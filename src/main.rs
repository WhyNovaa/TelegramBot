mod parse;
mod tg_tools;

use std::env;
use frankenstein::{AsyncApi, AsyncTelegramApi};
use tokio;
use dotenv::dotenv;


use crate::tg_tools::handlers::handle_updates;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN must be set");
    let api = AsyncApi::new(token.as_str());

    handle_updates(&api).await;
}



