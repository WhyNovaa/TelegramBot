mod parse;
mod tg_tools;
mod sqlite;

use std::env;
use frankenstein::AsyncApi;
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



