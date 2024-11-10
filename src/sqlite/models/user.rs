pub enum Page {
    ChooseRoute,
    ChooseTrain,
    Waiting
}

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