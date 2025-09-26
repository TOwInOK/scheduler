use frankenstein::{
    AsyncTelegramApi,
    client_reqwest::Bot,
    methods::{DeleteMessageParams, SendMessageParams},
    types::{CallbackQuery, MaybeInaccessibleMessage, ReplyMarkup},
};
use tracing::error;

use crate::{
    cells::cell::{groups::Groups, subgroup::SubGroup},
    telegram::{TGState, keyboard::main_commands_keyboard},
    user::User,
};

pub enum Callback {
    SelectGroup(Groups),
}

pub async fn on_callback(callback_query: CallbackQuery, bot: Bot, state: TGState) {
    if let (Some(data), Some(MaybeInaccessibleMessage::Message(message))) =
        (&callback_query.data, &callback_query.message)
    {
        let group_selection = match data.as_str() {
            "Select-Group-1A" => Some(Callback::SelectGroup(Groups::First(SubGroup::A))),
            "Select-Group-1B" => Some(Callback::SelectGroup(Groups::First(SubGroup::B))),
            "Select-Group-2A" => Some(Callback::SelectGroup(Groups::Second(SubGroup::A))),
            "Select-Group-2B" => Some(Callback::SelectGroup(Groups::Second(SubGroup::B))),
            //
            _ => {
                error!("Received unknown callback data: {}", data);
                None
            }
        };

        if let Some(Callback::SelectGroup(groups)) = group_selection {
            let user = User::new(callback_query.from.id as i64, groups);

            if let Err(e) = user.create_user(state.pool.clone()).await {
                match &e {
                    // Change group if user already exists
                    crate::error::Error::SQL(sqlx::Error::Database(db_err)) => {
                        if db_err.is_unique_violation() {
                            if let Err(err) =
                                user.update_selected_group(state.pool.clone(), groups).await
                            {
                                error!(
                                    "Failed to update user's selected group (id={}): {:?}",
                                    user.id, err
                                );
                            }
                        } else {
                            error!(
                                "Database error while creating user (id={}): {:?}",
                                user.id, db_err
                            );
                        }
                    }
                    _ => error!(
                        "Unexpected error while creating user (id={}): {:?}",
                        user.id, e
                    ),
                }
            }

            if let Err(err) = bot
                .delete_message(
                    &DeleteMessageParams::builder()
                        .chat_id(message.chat.id)
                        .message_id(message.message_id)
                        .build(),
                )
                .await
            {
                error!(
                    "Failed to delete message (chat_id={}): {:?}",
                    message.chat.id, err
                );
            }

            let send_message_params = SendMessageParams::builder()
                .chat_id(message.chat.id)
                .text("Отлично! Твоя группа выбрана. Теперь можешь использовать кнопки ниже, чтобы просмотреть расписание:")
                .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(main_commands_keyboard()))
                .build();

            if let Err(err) = bot.send_message(&send_message_params).await {
                error!(
                    "Failed to send confirmation message (chat_id={}): {:?}",
                    message.chat.id, err
                );
            }
        }
    }
}
