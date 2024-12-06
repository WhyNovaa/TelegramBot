use sqlx::{sqlite::SqlitePool, Executor};
use tokio::fs::{metadata, File};


use crate::sqlite::models::user::{User, Page};


const DATABASE_STRUCT: &str = "CREATE TABLE IF NOT EXISTS Users (
            chat_id INTEGER PRIMARY KEY,
            page TEXT NOT NULL CHECK(PAGE in ('ChooseRoute', 'ChooseTrain', 'Waiting')),
            date TEXT,
            waiting_train_number TEXT,
            waiting_amount INTEGER
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

pub async fn add_user(pool: &SqlitePool, user: &User) -> Result<bool, sqlx::Error> {
    let req = sqlx::query(
        "INSERT INTO Users(chat_id, page, date, waiting_train_number, waiting_amount)
            VALUES(?, ?, ?, ?, ?)"
    )
        .bind(user.chat_id)
        .bind(user.page.as_str())
        .bind(user.date.clone())
        .bind(None::<String>)
        .bind(None::<String>)
        .execute(pool)
        .await?;

    Ok(req.rows_affected() == 1)
}

pub async fn get_user_chat_id(pool: &SqlitePool, chat_id: &i64) -> Result<User,sqlx::Error> {
    let req = sqlx::query("SELECT * FROM Users WHERE chat_id=?")
        .bind(chat_id)
        .fetch_one(pool)
        .await?;

    let user = User::from_sqlite_row(&req);

    Ok(user)
}

pub async fn change_user_page(pool: &SqlitePool, chat_id: &i64, page: &Page) -> Result<bool, sqlx::Error> {
    let req = sqlx::query("UPDATE Users SET page=? WHERE chat_id=?")
        .bind(page.as_str())
        .bind(chat_id)
        .execute(pool)
        .await?;
    Ok(req.rows_affected() == 1)
}

pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let req = sqlx::query("SELECT * FROM Users")
        .fetch_all(pool)
        .await?;

    let vec = req.iter()
        .map(|row| User::from_sqlite_row(row))
        .collect::<Vec<User>>();
    Ok(vec)
}

