pub mod callback;
pub mod message;

use callback::on_callback;
use frankenstein::{client_reqwest::Bot, updates::UpdateContent};
use message::on_message;

use super::TGState;

pub async fn on_update(update: UpdateContent, bot: &Bot, state: TGState) {
    match update {
        UpdateContent::Message(message) => {
            let bot = bot.clone();
            tokio::spawn(async move { on_message(*message, bot, state).await });
        }
        UpdateContent::CallbackQuery(callback_query) => {
            let bot = bot.clone();
            tokio::spawn(async move { on_callback(*callback_query, bot, state).await });
        }
        _ => {}
    };
}
