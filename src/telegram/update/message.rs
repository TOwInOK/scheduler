use frankenstein::{
    AsyncTelegramApi,
    client_reqwest::Bot,
    methods::SendMessageParams,
    types::{Message, ReplyMarkup, ReplyParameters},
};
use time::{Duration, UtcDateTime};
use tracing::error;

use crate::{
    cells::{render_cells, render_cells_week},
    telegram::{
        TGState,
        keyboard::{group_inline_keyboard, main_commands_keyboard},
    },
};

pub async fn on_message(message: Message, bot: Bot, state: TGState) {
    if let Some(text) = message.text {
        match text.as_str() {
            "/start" => {
                let reply_parameters = ReplyParameters::builder()
                    .message_id(message.message_id)
                    .build();
                let send_message_params = SendMessageParams::builder()
                    .chat_id(message.chat.id)
                    .text(
                        "Привет! Я бот для просмотра расписания. Выбери свою группу, чтобы начать:",
                    )
                    .reply_parameters(reply_parameters)
                    .reply_markup(ReplyMarkup::InlineKeyboardMarkup(group_inline_keyboard()))
                    .build();

                if let Err(error) = bot.send_message(&send_message_params).await {
                    error!("Failed to send message: {error:?}");
                }
            }
            "/schedule" => {
                let reply_parameters = ReplyParameters::builder()
                    .message_id(message.message_id)
                    .build();
                let send_message_params = SendMessageParams::builder()
                    .chat_id(message.chat.id)
                    .text("Отлично! Используй кнопки ниже, чтобы просмотреть расписание:")
                    .reply_parameters(reply_parameters)
                    .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(main_commands_keyboard()))
                    .build();

                if let Err(error) = bot.send_message(&send_message_params).await {
                    error!("Failed to send message: {error:?}");
                }
            }
            "Сегодня" => {
                if let Some(id) = message.from.map(|x| x.id) {
                    if let Some(group) = state.users.lock().await.get(&id).map(|x| x.group) {
                        let date = UtcDateTime::now().date();
                        let text = render_cells(
                            &state.cells.as_ref().filter_and_sort(group, date),
                            group,
                            date,
                        );
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(text)
                            .build();

                        if let Err(error) = bot.send_message(&send_message_params).await {
                            error!("Failed to send message: {error:?}");
                        }
                    }
                } else {
                    let send_message_params = SendMessageParams::builder()
                        .chat_id(message.chat.id)
                        .text("Обратись к /start")
                        .build();

                    if let Err(error) = bot.send_message(&send_message_params).await {
                        error!("Failed to send message: {error:?}");
                    }
                }
            }
            "Завтра" => {
                if let Some(id) = message.from.map(|x| x.id) {
                    if let Some(group) = state.users.lock().await.get(&id).map(|x| x.group) {
                        let date = UtcDateTime::now().date() + Duration::days(1);
                        let text = render_cells(
                            &state.cells.as_ref().filter_and_sort(group, date),
                            group,
                            date,
                        );
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(text)
                            .build();

                        if let Err(error) = bot.send_message(&send_message_params).await {
                            error!("Failed to send message: {error:?}");
                        }
                    }
                } else {
                    let send_message_params = SendMessageParams::builder()
                        .chat_id(message.chat.id)
                        .text("Обратись к /start")
                        .build();

                    if let Err(error) = bot.send_message(&send_message_params).await {
                        error!("Failed to send message: {error:?}");
                    }
                }
            }
            "Вчера" => {
                if let Some(id) = message.from.map(|x| x.id) {
                    if let Some(group) = state.users.lock().await.get(&id).map(|x| x.group) {
                        let date = UtcDateTime::now().date() - Duration::days(1);
                        let text = render_cells(
                            &state.cells.as_ref().filter_and_sort(group, date),
                            group,
                            date,
                        );
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(text)
                            .build();

                        if let Err(error) = bot.send_message(&send_message_params).await {
                            error!("Failed to send message: {error:?}");
                        }
                    }
                } else {
                    let send_message_params = SendMessageParams::builder()
                        .chat_id(message.chat.id)
                        .text("Обратись к /start")
                        .build();

                    if let Err(error) = bot.send_message(&send_message_params).await {
                        error!("Failed to send message: {error:?}");
                    }
                }
            }
            "Неделя" => {
                if let Some(id) = message.from.map(|x| x.id) {
                    if let Some(group) = state.users.lock().await.get(&id).map(|x| x.group) {
                        let date = UtcDateTime::now().date();
                        let text = render_cells_week(
                            &state.cells.as_ref().filtered_week(group, date),
                            group,
                        );
                        let send_message_params = SendMessageParams::builder()
                            .chat_id(message.chat.id)
                            .text(text)
                            .build();

                        if let Err(error) = bot.send_message(&send_message_params).await {
                            error!("Failed to send message: {error:?}");
                        }
                    }
                } else {
                    let reply_parameters = ReplyParameters::builder()
                        .message_id(message.message_id)
                        .build();
                    let send_message_params = SendMessageParams::builder()
                        .chat_id(message.chat.id)
                        .text("Обратись к /start")
                        .reply_parameters(reply_parameters)
                        .build();

                    if let Err(error) = bot.send_message(&send_message_params).await {
                        error!("Failed to send message: {error:?}");
                    }
                }
            }
            "" => {}
            _ => {
                error!("Received unknown callback data: {}", text);
            }
        }
    }
}
