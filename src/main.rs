mod cells;
pub mod dicts;
pub mod error;
pub mod parser;
pub mod telegram;

use std::{collections::HashMap, sync::Arc};

use dotenvy_macro::dotenv;
use frankenstein::{AsyncTelegramApi, client_reqwest::Bot, methods::GetUpdatesParams};
use parser::load_cells_store;
use telegram::{State, update::on_update};
use tokio::sync::Mutex;
use tracing::Level;
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
    dotenvy::dotenv().unwrap();
    let token = dotenv!("BOT_TOKEN");
    let bot = Bot::new(token);
    let cells = load_cells_store().await.expect("fail to load store");
    let state = Arc::new(State {
        users: Arc::new(Mutex::new(HashMap::new())),
        cells: Arc::new(cells),
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
}
