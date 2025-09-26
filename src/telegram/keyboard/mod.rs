use frankenstein::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup,
};

pub fn group_inline_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::builder()
        .inline_keyboard(vec![
            vec![
                InlineKeyboardButton::builder()
                    .text("1A")
                    .callback_data("Select-Group-1A")
                    .build(),
                InlineKeyboardButton::builder()
                    .text("1B")
                    .callback_data("Select-Group-1B")
                    .build(),
            ],
            vec![
                InlineKeyboardButton::builder()
                    .text("2A")
                    .callback_data("Select-Group-2A")
                    .build(),
                InlineKeyboardButton::builder()
                    .text("2B")
                    .callback_data("Select-Group-2B")
                    .build(),
            ],
        ])
        .build()
}

pub fn main_commands_keyboard() -> ReplyKeyboardMarkup {
    ReplyKeyboardMarkup::builder()
        .keyboard(vec![
            vec![
                KeyboardButton::builder().text("Вчера").build(),
                KeyboardButton::builder().text("Завтра").build(),
                KeyboardButton::builder().text("Сегодня").build(),
                KeyboardButton::builder().text("Неделя").build(),
            ],
            vec![
                KeyboardButton::builder().text("След.Вчера").build(),
                KeyboardButton::builder().text("След.Завтра").build(),
                KeyboardButton::builder().text("След.Сегодня").build(),
                KeyboardButton::builder().text("След.Неделя").build(),
            ],
            vec![
                KeyboardButton::builder().text("Пред.Вчера").build(),
                KeyboardButton::builder().text("Пред.Завтра").build(),
                KeyboardButton::builder().text("Пред.Сегодня").build(),
                KeyboardButton::builder().text("Пред.Неделя").build(),
            ],
        ])
        .build()
}
