#![forbid(unsafe_code)]

mod cells;
pub mod database;
pub mod dicts;
pub mod error;
pub mod parser;
pub mod telegram;
pub mod user;

use std::{collections::HashMap, sync::Arc};

use frankenstein::{AsyncTelegramApi, client_reqwest::Bot, methods::GetUpdatesParams};
use parser::load_cells_store;
use telegram::{State, update::on_update};
use tokio::sync::Mutex;
use tracing::Level;

use crate::database::{create_default_if_not_exists, execute_pool};
fn init_logger(level: Level) {
    use tracing_subscriber::FmtSubscriber;

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .compact()
            .with_max_level(level)
            .without_time()
            .finish(),
    )
    .expect("Fail to set global default subscriber");
}

#[tokio::main]
async fn main() {
    init_logger(Level::INFO);
    dotenvy::dotenv().ok();
    let pool = execute_pool().await.unwrap();
    create_default_if_not_exists(pool.clone()).await.unwrap();

    let token = std::env::var("BOT_TOKEN")
        .expect("BOT_TOKEN must be set either in a .env file or as an environment variable");
    let bot = Bot::new(&token);
    let cells = load_cells_store().await.expect("fail to load store");
    let state = Arc::new(State {
        users: Arc::new(Mutex::new(HashMap::new())),
        cells: Arc::new(cells),
        pool,
    });
    let mut update_params = GetUpdatesParams::builder().build();

    loop {
        let result = bot.get_updates(&update_params).await;
        match result {
            Ok(response) => {
                for update in response.result {
                    on_update(update.content, &bot, state.clone()).await;
                    update_params.offset = Some(i64::from(update.update_id) + 1);
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }
    // TODO: add gracefull shutdown
    // - close database connection
    // - end telegram bot tasks
    // - stop telegram bot
}
