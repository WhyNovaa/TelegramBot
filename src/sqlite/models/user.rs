use sqlx::{FromRow, Row};
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

    pub fn back(&self) -> Page {
        match self {
            Page::ChooseRoute => Page::ChooseRoute,
            Page::ChooseTrain => Page::ChooseRoute,
            Page::Waiting => Page::ChooseTrain,
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

#[derive(Debug)]
pub struct Waiting {
    pub train_number: String,
    pub amount: u64,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub chat_id: i64,
    pub page: Page,
    pub date: String,
    pub waiting: Option<Waiting>,
}

impl User {
    pub fn new(chat_id: i64, page: Page, date: String) -> Self {
        Self {
            chat_id,
            page,
            date,
           waiting: None,
        }
    }
    pub fn from_sqlite_row(row: &SqliteRow) -> Self {
        Self {
            chat_id: row.get("chat_id"),
            page: Page::from_str(row.get("page")).unwrap(),
            date: row.get("date"),
            waiting: Some(Waiting {
                train_number: row.get("train_number"),
                amount: row.get("amount")
            }),
        }
    }
}