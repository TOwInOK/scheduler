use frankenstein::{
    AsyncTelegramApi, client_reqwest::Bot, methods::SetMyCommandsParams, types::BotCommand,
};

pub async fn set_commands(bot: &Bot) {
    let commands = SetMyCommandsParams::builder()
        .commands(vec![
            // Основные команды
            BotCommand::builder()
                .command("/start")
                .description("Показать меню выбора группы")
                .build(),
            BotCommand::builder()
                .command("/schedule")
                .description("Показать клавиатуру с выбором расписания")
                .build(),
            BotCommand::builder()
                .command("/faq")
                .description("Показать faq")
                .build(),
            // Команды расписания
            BotCommand::builder()
                .command("/daytime_current")
                .description("Показать расписание на текущий день")
                .build(),
            BotCommand::builder()
                .command("/daytime_before")
                .description("Показать расписание на предыдущий день")
                .build(),
            BotCommand::builder()
                .command("/daytime_next")
                .description("Показать расписание на следующий день")
                .build(),
            BotCommand::builder()
                .command("/daytime_week")
                .description("Показать расписание на неделю")
                .build(),
            // Команды расписания на следующую неделю
            BotCommand::builder()
                .command("/next_week_current")
                .description("Показать расписание на сегоднешний день следующей недели")
                .build(),
            BotCommand::builder()
                .command("/next_week_next")
                .description("Показать расписание на следующий день следующей недели")
                .build(),
            BotCommand::builder()
                .command("/next_week_before")
                .description("Показать расписание на предыдущий день следующей недели")
                .build(),
            BotCommand::builder()
                .command("/next_week_week")
                .description("Показать расписание на следующую неделю")
                .build(),
            // Команды расписания на предыдущую неделю
            BotCommand::builder()
                .command("/prev_week_current")
                .description("Показать расписание на сегоднешний день предыдущей недели")
                .build(),
            BotCommand::builder()
                .command("/prev_week_next")
                .description("Показать расписание на следующий день предыдущей недели")
                .build(),
            BotCommand::builder()
                .command("/prev_week_before")
                .description("Показать расписание на предыдущий день предыдущей недели")
                .build(),
            BotCommand::builder()
                .command("/prev_week_week")
                .description("Показать расписание на предыдущую неделю")
                .build(),
        ])
        .build();
    bot.set_my_commands(&commands).await.unwrap();
}
