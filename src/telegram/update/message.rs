use frankenstein::{
    AsyncTelegramApi,
    client_reqwest::Bot,
    methods::SendMessageParams,
    types::{Message, ReplyMarkup, ReplyParameters},
};
use time::{Date, Duration, UtcDateTime, UtcOffset};
use tracing::{error, warn};

use crate::{
    cells::{render_cells, render_cells_week},
    telegram::{
        TGState,
        keyboard::{group_inline_keyboard, main_commands_keyboard},
    },
    user::User,
};

pub enum DayTime {
    Before,
    Current,
    Next,
    Week,
    Month,
}

enum OnMessageAction {
    Start,
    Schedule,
    FAQ,
    DayTime(DayTime),
    NextWeek(DayTime),
    PastWeek(DayTime),
    Unknown,
}

pub async fn on_message(message: Message, bot: Bot, state: TGState) {
    if let Some(text) = &message.text {
        let token: OnMessageAction = match text.as_str() {
            "/start" => OnMessageAction::Start,
            "/schedule" => OnMessageAction::Schedule,
            "/faq" => OnMessageAction::FAQ,
            "Сегодня" | "/daytime_current" => OnMessageAction::DayTime(DayTime::Current),
            "Вчера" | "/daytime_before" => OnMessageAction::DayTime(DayTime::Before),
            "Завтра" | "/daytime_next" => OnMessageAction::DayTime(DayTime::Next),
            "Неделя" | "/daytime_week" => OnMessageAction::DayTime(DayTime::Week),
            "Месяц" | "/daytime_month" => OnMessageAction::DayTime(DayTime::Month),
            "След.Сегодня" | "/next_week_current" => {
                OnMessageAction::NextWeek(DayTime::Current)
            }
            "След.Вчера" | "/next_week_before" => {
                OnMessageAction::NextWeek(DayTime::Before)
            }
            "След.Завтра" | "/next_week_next" => OnMessageAction::NextWeek(DayTime::Next),
            "След.Неделя" | "/next_week_week" => OnMessageAction::NextWeek(DayTime::Week),
            "След.Месяц" | "/next_week_month" => OnMessageAction::NextWeek(DayTime::Month),
            "Пред.Сегодня" | "/prev_week_current" => {
                OnMessageAction::PastWeek(DayTime::Current)
            }
            "Пред.Вчера" | "/prev_week_before" => {
                OnMessageAction::PastWeek(DayTime::Before)
            }
            "Пред.Завтра" | "/prev_week_next" => OnMessageAction::PastWeek(DayTime::Next),
            "Пред.Неделя" | "/prev_week_week" => OnMessageAction::PastWeek(DayTime::Week),
            "Пред.Месяц" | "/prev_week_month" => OnMessageAction::PastWeek(DayTime::Month),
            _ => OnMessageAction::Unknown,
        };
        match token {
            OnMessageAction::Start => {
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
            OnMessageAction::Schedule => {
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
            OnMessageAction::DayTime(daytime) => match daytime {
                DayTime::Before => {
                    handle_daytime(&bot, &state, &message, |u, date| {
                        let date = date - Duration::days(1);
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Current => {
                    handle_daytime(&bot, &state, &message, |u, date| {
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Next => {
                    handle_daytime(&bot, &state, &message, |u, date| {
                        let date = date + Duration::days(1);
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Week => {
                    handle_daytime(&bot, &state, &message, |u, date| {
                        render_cells_week(
                            &state.cells.as_ref().filtered_week(u.selected_group, date),
                            u.selected_group,
                        )
                    })
                    .await;
                }
                DayTime::Month => warn!("Not implemented: Month"),
            },
            OnMessageAction::Unknown => {
                error!("Received unknown callback data: {}", text);
                send_message(
                    &bot,
                    message.chat.id,
                    "Ну и что мне делать с этим?\nПосмотри меню.",
                )
                .await;
            }
            OnMessageAction::NextWeek(daytime) => match daytime {
                DayTime::Before => {
                    handle_daytime_next_week(&bot, &state, &message, |u, date| {
                        let date = date - Duration::days(1);
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Current => {
                    handle_daytime_next_week(&bot, &state, &message, |u, date| {
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Next => {
                    handle_daytime_next_week(&bot, &state, &message, |u, date| {
                        let date = date + Duration::days(1);
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Week => {
                    handle_daytime_next_week(&bot, &state, &message, |u, date| {
                        render_cells_week(
                            &state.cells.as_ref().filtered_week(u.selected_group, date),
                            u.selected_group,
                        )
                    })
                    .await;
                }
                DayTime::Month => warn!("Not implemented: Month"),
            },
            OnMessageAction::PastWeek(daytime) => match daytime {
                DayTime::Before => {
                    handle_daytime_prev_week(&bot, &state, &message, |u, date| {
                        let date = date - Duration::days(1);
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Current => {
                    handle_daytime_prev_week(&bot, &state, &message, |u, date| {
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Next => {
                    handle_daytime_prev_week(&bot, &state, &message, |u, date| {
                        let date = date + Duration::days(1);
                        render_cells(
                            &state.cells.as_ref().filter_and_sort(u.selected_group, date),
                            u.selected_group,
                            date,
                        )
                    })
                    .await;
                }
                DayTime::Week => {
                    handle_daytime_prev_week(&bot, &state, &message, |u, date| {
                        render_cells_week(
                            &state.cells.as_ref().filtered_week(u.selected_group, date),
                            u.selected_group,
                        )
                    })
                    .await;
                }
                DayTime::Month => warn!("Not implemented: Month"),
            },
            OnMessageAction::FAQ => {
                send_message(
                    &bot,
                    message.chat.id,
                    "
                # Обозначение типа занятия\n- Лекция -> 🟠\n- Практика -> 🟢\n\n# Тип недели\n- Н -> нечётная\n- Ч -> чётная\n
                ",
                )
                .await
            }
        }
    }
}

async fn send_message(bot: &Bot, id: i64, text: &str) {
    let send_message_params = SendMessageParams::builder().chat_id(id).text(text).build();

    if let Err(error) = bot.send_message(&send_message_params).await {
        error!("Failed to send message: {error:?}");
    }
}
async fn handle_daytime<F>(bot: &Bot, state: &TGState, message: &Message, f: F)
where
    F: FnOnce(&User, Date) -> String,
{
    use time_tz::{OffsetDateTimeExt, timezones};
    if let Some(id) = message.from.as_ref().map(|x| x.id as i64) {
        match User::get_user_by_id(state.pool.clone(), id).await {
            Ok(u) => {
                let date = UtcDateTime::now()
                    .to_offset(UtcOffset::UTC)
                    .to_timezone(timezones::db::asia::NOVOSIBIRSK)
                    .date();
                let text = f(&u, date);
                send_message(bot, message.chat.id, &text).await;
            }
            Err(e) => {
                error!("Failed to get user by id: {e:?}");
                send_message(bot, message.chat.id, "Обратись к /start").await;
            }
        }
    }
}

async fn handle_daytime_next_week<F>(bot: &Bot, state: &TGState, message: &Message, f: F)
where
    F: FnOnce(&User, Date) -> String,
{
    use time_tz::{OffsetDateTimeExt, timezones};
    if let Some(id) = message.from.as_ref().map(|x| x.id as i64) {
        match User::get_user_by_id(state.pool.clone(), id).await {
            Ok(u) => {
                let date = (UtcDateTime::now() + Duration::weeks(1))
                    .to_offset(UtcOffset::UTC)
                    .to_timezone(timezones::db::asia::NOVOSIBIRSK)
                    .date();
                let text = f(&u, date);
                send_message(bot, message.chat.id, &text).await;
            }
            Err(e) => {
                error!("Failed to get user by id: {e:?}");
                send_message(bot, message.chat.id, "Обратись к /start").await;
            }
        }
    }
}

async fn handle_daytime_prev_week<F>(bot: &Bot, state: &TGState, message: &Message, f: F)
where
    F: FnOnce(&User, Date) -> String,
{
    use time_tz::{OffsetDateTimeExt, timezones};
    if let Some(id) = message.from.as_ref().map(|x| x.id as i64) {
        match User::get_user_by_id(state.pool.clone(), id).await {
            Ok(u) => {
                let date = (UtcDateTime::now() - Duration::weeks(1))
                    .to_offset(UtcOffset::UTC)
                    .to_timezone(timezones::db::asia::NOVOSIBIRSK)
                    .date();
                let text = f(&u, date);
                send_message(bot, message.chat.id, &text).await;
            }
            Err(e) => {
                error!("Failed to get user by id: {e:?}");
                send_message(bot, message.chat.id, "Обратись к /start").await;
            }
        }
    }
}
