
use tokio_rusqlite::{Connection, params};
use crate::sqlite::models::user::{User, Page};
pub async fn create_db() -> Connection {
    let conn = Connection::open("users.db").await.expect("DB initialization error");

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Users (
                    chat_id INTEGER PRIMARY KEY,
                    page TEXT NOT NULL CHECK(PAGE in ('ChooseRoute', 'ChooseTrain', 'Waiting')),
                    date TEXT,
                    waiting_train_number TEXT
                )",
            [],
        ).unwrap();
        Ok(())
    }).await.expect("DB creation error");

    conn
}

pub async fn add_user(conn: &Connection, user: User) {
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO Users(chat_id, page, date, waiting_train_number)
                 VALUES(?, ?, ?, ?)",
            params![
                         user.chat_id,
                         match user.page {
                             Page::ChooseRoute => {"ChooseRoute"},
                             Page::ChooseTrain => {"ChooseTrain"},
                             Page::Waiting => {"Waiting"}
                            },
                         user.date,
                         user.waiting_train_number
                     ]
        ).unwrap();
        Ok(())
    }).await.expect("DB addition error");
}