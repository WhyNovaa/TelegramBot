
use sqlx::{sqlite::SqlitePool,Executor};
use crate::sqlite::models::user::{User, Page};

use tokio::fs::{metadata, File};

pub async fn create_path_if_necessary() {
    let file_path = "users.db";
    match metadata(file_path).await {
        Ok(_) => {}
        Err(_) => {
            File::create(file_path).await.unwrap();
        }
    }
}
pub async fn create_db() -> SqlitePool {
    create_path_if_necessary().await;
    let pool = SqlitePool::connect("sqlite:./users.db").await
        .expect("DB initialization error");

    pool.execute(
        "CREATE TABLE IF NOT EXISTS Users (
            chat_id INTEGER PRIMARY KEY,
            page TEXT NOT NULL CHECK(PAGE in ('ChooseRoute', 'ChooseTrain', 'Waiting')),
            date TEXT,
            waiting_train_number TEXT
        )"
    ).await.expect("DB creation error");

    pool
}

pub async fn add_user(pool: &SqlitePool, user: User) {
    sqlx::query(
        "INSERT INTO Users(chat_id, page, date, waiting_train_number)
            VALUES(?, ?, ?, ?)"
    )
        .bind(user.chat_id)
        .bind(match user.page {
            Page::ChooseRoute => "ChooseRoute",
            Page::ChooseTrain => "ChooseTrain",
            Page::Waiting => "Waiting",
        })
        .bind(user.date)
        .bind(user.waiting_train_number)
        .execute(pool)
        .await
        .expect("DB addition error");
}
