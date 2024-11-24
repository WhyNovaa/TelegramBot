use std::collections::HashMap;
use std::process::exit;
use std::sync::Arc;
use tokio::sync::Mutex;

use frankenstein::{AsyncApi, AsyncTelegramApi, GetUpdatesParams, InlineKeyboardButton, Message, ReplyMarkup, ReplyParameters, SendMessageParams, UpdateContent};
use frankenstein::objects::InlineKeyboardMarkup;

use reqwest::Client;
use chrono;
use sqlx::SqlitePool;
use crate::parse::parsing::get_all_trains;
use crate::parse::models::train::Train;

use crate::sqlite::db_tools::{add_user, create_db, get_all_users, get_user_chat_id};
use crate::sqlite::models::user::{Page, User};

pub async fn handle_updates(api: &AsyncApi) {

    let conn: SqlitePool;
    match create_db().await {
        Ok(c) => {conn = c},
        Err(e) => {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    }


    println!("{:?}", get_all_users(&conn).await.unwrap());



    /*let client = Client::new();
    let date = chrono::offset::Local::now().date_naive().to_string();

    let mut offset: i64 = 0;

    loop {
        let map = Arc::new(Mutex::new(get_all_trains(&client, date.as_str()).await));

        let m_guard = map.lock().await;

        println!("{:?}", m_guard.values());





        let update_params = GetUpdatesParams::builder()
            .offset(offset)
            .build();

        let updates = api.get_updates(&update_params).await.unwrap();

        for update in updates.result {
            let content = update.content;

            match content {
                UpdateContent::Message(message) => {


                    //m_guard.entry(message.chat.id).or_insert(User::new(&message.chat.id, Page::ChooseRoute, date.clone(), None));
                    if let Some(text) = message.text {
                        if text == "/train" {
                            let api_clone = api.clone();
                            let chat_id = message.chat.id.clone();
                            let map_clone = Arc::clone(&map);

                            tokio::spawn(async move {
                                //process_message(message_clone, api_clone, "Hui".to_string()).await;
                                send_message_with_inline_trains(&api_clone, chat_id, map_clone).await;
                            });
                        }
                        else if text == "/date" {

                        }
                    }

                    offset = (update.update_id + 1) as i64;
                }
                UpdateContent::CallbackQuery(callback_query) => {
                    if let Some(data) = callback_query.data {
                        let parts: Vec<&str> = data.split('|').collect();

                        if parts.len() == 2 {

                        }
                    }
                }
                _ => {}
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }*/
}

async fn process_message(message: Message, api: AsyncApi, text: String) {
    let reply_parameters = ReplyParameters::builder()
        .message_id(message.message_id)
        .build();
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(text)
        .reply_parameters(reply_parameters)
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

async fn send_message_with_inline_trains(api: &AsyncApi, chat_id: i64, map: Arc<Mutex<HashMap<String, Train>>>) {
    let m_guard = map.lock().await;
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

    api.send_message(&params).await.expect("Could send message");
}