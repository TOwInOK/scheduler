use frankenstein::{
    AsyncTelegramApi,
    client_reqwest::Bot,
    methods::{DeleteMessageParams, SendMessageParams},
    types::{CallbackQuery, MaybeInaccessibleMessage, ReplyMarkup},
};
use tracing::error;

use crate::{
    cells::cell::{groups::Groups, subgroup::SubGroup},
    telegram::{TGState, UserState, keyboard::main_commands_keyboard},
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
            _ => {
                error!("Received unknown callback data: {}", data);
                None
            }
        };

        if let Some(callback) = group_selection {
            match callback {
                Callback::SelectGroup(groups) => {
                    {
                        state
                            .users
                            .lock()
                            .await
                            .insert(callback_query.from.id, UserState { group: groups });
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
                        error!("Failed to edit message text: {:?}", err);
                    }

                    let send_message_params = SendMessageParams::builder()
                        .chat_id(message.chat.id)
                        .text(
                            "Отлично! Твоя группа выбрана. Теперь можешь использовать кнопки ниже, чтобы просмотреть расписание:",
                        )
                        .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(main_commands_keyboard()))
                        .build();

                    if let Err(error) = bot.send_message(&send_message_params).await {
                        error!("Failed to send message: {error:?}");
                    }
                }
            }
        }
    }
}
