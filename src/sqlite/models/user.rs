use sqlx::{Database, FromRow, Sqlite};
use sqlx::{Decode, Type};
use std::str::FromStr;
use sqlx::error::BoxDynError;

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
        Self{
            chat_id,
            page,
            date,
            waiting_train_number,
        }
    }
}