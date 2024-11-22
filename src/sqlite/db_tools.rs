use std::str::FromStr;
use sqlx::{sqlite::SqlitePool, Executor, Row};
use sqlx::sqlite::SqliteError;
use crate::sqlite::models::user::{User, Page};

use tokio::fs::{metadata, File};


const DATABASE_STRUCT: &str = "CREATE TABLE IF NOT EXISTS Users (
            chat_id INTEGER PRIMARY KEY,
            page TEXT NOT NULL CHECK(PAGE in ('ChooseRoute', 'ChooseTrain', 'Waiting')),
            date TEXT,
            waiting_train_number TEXT
        )";


pub async fn create_path_if_necessary() {
    let file_path = "users.db";
    match metadata(file_path).await {
        Ok(_) => {}
        Err(_) => {
            File::create(file_path).await.unwrap();
        }
    }
}
pub async fn create_db() -> Result<SqlitePool, sqlx::Error> {
    create_path_if_necessary().await;
    let pool = SqlitePool::connect("sqlite:./users.db").await
        .expect("DB initialization error");

    pool.execute(DATABASE_STRUCT)
        .await?;

    Ok(pool)

}

pub async fn add_user(pool: &SqlitePool, user: User) -> Result<bool, sqlx::Error> {
    sqlx::query(
        "INSERT INTO Users(chat_id, page, date, waiting_train_number)
            VALUES(?, ?, ?, ?)"
    )
        .bind(user.chat_id)
        .bind(user.page.as_str())
        .bind(user.date)
        .bind(user.waiting_train_number)
        .execute(pool)
        .await?;

    Ok(true)
}

pub async fn get_user_chat_id(pool: &SqlitePool, chat_id: &i64) -> Result<User,sqlx::Error> {
    let req = sqlx::query("SELECT * FROM Users WHERE chat_id=?")
        .bind(chat_id)
        .fetch_one(pool)
        .await?;

    let chat_id = req.get("chat_id");
    let page = Page::from_str(req.get("page")).unwrap();
    let date = req.get("date");
    let waiting_train_number = req.get("waiting_train_number");

    let user = User::new(chat_id, page, date, waiting_train_number);

    Ok(user)
}
