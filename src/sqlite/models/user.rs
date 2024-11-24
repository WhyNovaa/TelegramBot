use sqlx::{Database, FromRow, Row};
use std::str::FromStr;
use sqlx::sqlite::SqliteRow;

#[derive(Debug)]
pub enum Page {
    ChooseRoute,
    ChooseTrain,
    Waiting
}
impl Page {
    pub fn as_str(&self) -> &'static str {
        match self {
            Page::ChooseRoute => "ChooseRoute",
            Page::ChooseTrain => "ChooseTrain",
            Page::Waiting => "Waiting",
        }
    }
}
impl FromStr for Page {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ChooseRoute" => Ok(Page::ChooseRoute),
            "ChooseTrain" => Ok(Page::ChooseTrain),
            "Waiting" => Ok(Page::Waiting),
            _ => Err(format!["Invalid value: {} for Page", s])
        }
    }
}


#[derive(Debug, FromRow)]
pub struct User {
    pub chat_id: i64,
    pub page: Page,
    pub date: String,
    pub waiting_train_number: Option<String>,
}

impl User {
    pub fn new(chat_id: i64, page: Page, date: String, waiting_train_number: Option<String>) -> Self {
        Self {
            chat_id,
            page,
            date,
            waiting_train_number,
        }
    }
    pub fn from_sqlite_row(row: &SqliteRow) -> Self {
        Self {
            chat_id: row.get("chat_id"),
            page: Page::from_str(row.get("page")).unwrap(),
            date: row.get("date"),
            waiting_train_number: row.get("waiting_train_number")
        }
    }
}