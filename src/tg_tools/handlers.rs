use std::collections::HashMap;
use std::process::exit;
use std::sync::Arc;
use tokio::sync::RwLock;
use frankenstein::{AsyncApi, AsyncTelegramApi, CallbackQuery, GetUpdatesParams, InlineKeyboardButton, Message, ReplyMarkup, SendMessageParams, UpdateContent};
use frankenstein::objects::InlineKeyboardMarkup;
use reqwest::Client;
use chrono;
use sqlx::SqlitePool;


use crate::parse::parsing::update_map_all_trains;
use crate::parse::models::train::Train;

use crate::sqlite::db_tools::{create_db, get_all_users};



pub async fn handle_updates(api: &AsyncApi) {

    let conn: SqlitePool;
    match create_db().await {
        Ok(c) => conn = c,
        Err(e) => {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    }

    let client = Client::new();
    let date = chrono::offset::Local::now().date_naive().to_string();

    let mut offset: i64 = 0;
    let trains: Arc<RwLock<HashMap<String, Train>>> = Arc::new(RwLock::new(HashMap::new()));


    let conn_clone = conn.clone();
    let trains_clone = Arc::clone(&trains);
    let api_clone = api.clone();
    tokio::spawn(async move {
        check_trains(api_clone, &conn_clone, trains_clone).await;
    });

    loop {
        let trains_clone = Arc::clone(&trains);
        update_map_all_trains(&client, trains_clone, date.as_str()).await;

        //let m_guard = map.read().await;
        //println!("{:?}", m_guard.values());

        let update_params = GetUpdatesParams::builder()
            .offset(offset)
            .build();

        let updates = api.get_updates(&update_params).await.unwrap();

        for update in updates.result {
            let content = update.content;

            match content {
                UpdateContent::Message(message) => {
                    let a = Arc::clone(&trains);
                    let api_clone = api.clone();

                    tokio::spawn(async move {
                        handle_message(api_clone, message, a).await;
                    });

                    offset = (update.update_id + 1) as i64;
                }
                UpdateContent::CallbackQuery(callback_query) => {
                    let a = Arc::clone(&trains);
                    let api_clone = api.clone();
                    tokio::spawn(async move {
                        handle_callback_query(api_clone, callback_query, a).await;
                    });
                }
                _ => {}
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}

async fn process_message(chat_id: i64, api: AsyncApi, text: String) {

    let send_message_params = SendMessageParams::builder()
        .chat_id(chat_id)
        .text(text)
        .build();
    if let Err(error) = api.send_message(&send_message_params).await {
        println!("Failed to send message: {error:?}");
    }
}
async fn get_button(train: &Train) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: format!{"{} {}({}) -- {}({})",
                      train.number.clone(),
                      train.start_station.clone(), train.start_time.clone(),
                      train.end_station.clone(), train.end_time.clone()},
        callback_data: Some(format!{"{}|{}","ChooseAction", train.number.clone()}),
        url: None,
        login_url: None,
        web_app: None,
        switch_inline_query: None,
        switch_inline_query_current_chat: None,
        switch_inline_query_chosen_chat: None,
        callback_game: None,
        pay: None,
    }
}

async fn send_message_with_inline_trains(api: AsyncApi, chat_id: i64, map: Arc<RwLock<HashMap<String, Train>>>) {
    let m_guard = map.read().await;
    let mut buttons = vec![Vec::new()];

    let mut sorted_buttons = m_guard.values().collect::<Vec<_>>();
    sorted_buttons.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    for train in sorted_buttons {
        buttons.push(vec![get_button(train).await]);
    }

    let inline_keyboard = InlineKeyboardMarkup {
        inline_keyboard: buttons,
    };

    let keyboard = ReplyMarkup::InlineKeyboardMarkup(inline_keyboard);

    let params = SendMessageParams::builder()
        .chat_id(chat_id).text("Выберите поезд:")
        .reply_markup(keyboard)
        .build();

    api.send_message(&params).await.expect("Couldn't send message");
}

async fn handle_message(api: AsyncApi, message: Message, trains: Arc<RwLock<HashMap<String, Train>>>) {
    if let Some(text) = message.text {
        if text == "/start" {
            //process_message(message_clone, api_clone, "Hui".to_string()).await;
            send_message_with_inline_trains(api.clone(), message.chat.id.clone(), Arc::clone(&trains)).await;
        }
        else {
            process_message(message.chat.id, api, String::from("Напишите /start, чтобы начать")).await;
        }
    }
}

async fn handle_callback_query(api: AsyncApi, callback_query: CallbackQuery, trains: Arc<RwLock<HashMap<String, Train>>>) {
    let data = callback_query.data.unwrap();


}

async fn check_trains(api: AsyncApi, pool: &SqlitePool, trains: Arc<RwLock<HashMap<String, Train>>>) {
    let users = get_all_users(pool).await.expect("Db getting error");
    let tr_guard = trains.read().await;
    for user in users {
        if let Some(waiting) = user.waiting {
            if let Some(train) = tr_guard.get(&waiting.train_number) {
                let mut res = 0_u64;
                let _ = train.tickets.iter().map(move |t| res += t.amount as u64);
                if res > waiting.amount {
                    process_message(user.chat_id, api.clone(), String::from("Появился билет!!")).await;
                }
            }
        }
    }
}