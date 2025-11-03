use teloxide::{prelude::*, types::CallbackQuery};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::telegram_bot::{Sessions, UserSession, SUBSCRIPTION_CODES};

// Callback query handler for inline keyboard buttons
pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    sessions: Sessions,
) -> ResponseResult<()> {
    let chat_id = q.message.as_ref().unwrap().chat.id;
    
    // Answer the callback query to remove the "loading" state
    bot.answer_callback_query(&q.id).await?;

    let data = match q.data {
        Some(ref data) => data.as_str(),
        None => return Ok(()),
    };

    match data {
        // Main menu buttons
        "redeem_code" => {
            crate::telegram_bot_handlers::handle_redeem_code(bot, chat_id, sessions).await?;
        }
        "my_subscription" => {
            crate::telegram_bot_handlers::handle_my_subscription(bot, chat_id, sessions).await?;
        }
        "crypt_file" => {
            crate::telegram_bot_handlers::handle_crypt_file(bot, chat_id, sessions).await?;
        }
        "faq" => {
            crate::telegram_bot_handlers::handle_faq(bot, chat_id).await?;
        }
        "support" => {
            crate::telegram_bot_handlers::handle_support(bot, chat_id).await?;
        }
        "main_menu" => {
            crate::telegram_bot_handlers::handle_main_menu(bot, chat_id).await?;
        }
        "view_updates" => {
            crate::telegram_bot_handlers::handle_view_updates(bot, chat_id).await?;
        }
        
        // Configuration toggle buttons
        "toggle_anti_debug" => {
            toggle_option(&sessions, chat_id, |config| config.anti_debug = !config.anti_debug);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_anti_vm" => {
            toggle_option(&sessions, chat_id, |config| config.anti_virtual_machine = !config.anti_virtual_machine);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_blacklist_cis" => {
            toggle_option(&sessions, chat_id, |config| config.blacklist_cis_countries = !config.blacklist_cis_countries);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_uac_bypass" => {
            toggle_option(&sessions, chat_id, |config| config.uac_bypass = !config.uac_bypass);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_single_instance" => {
            toggle_option(&sessions, chat_id, |config| config.single_instance = !config.single_instance);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_persistence" => {
            toggle_option(&sessions, chat_id, |config| config.run_on_startup = !config.run_on_startup);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_defender" => {
            toggle_option(&sessions, chat_id, |config| config.defender_exclusion = !config.defender_exclusion);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_drive_exclusion" => {
            toggle_option(&sessions, chat_id, |config| config.defender_exclude_drive = !config.defender_exclude_drive);
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "toggle_format" => {
            toggle_option(&sessions, chat_id, |config| {
                config.file_extension = if config.file_extension == "BAT" {
                    "EXE".to_string()
                } else {
                    "BAT".to_string()
                };
            });
            crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;
        }
        "start_build" => {
            // Start the build process
            crate::telegram_bot::start_build_process(bot.clone(), chat_id, sessions.clone()).await?;
        }
        _ => {}
    }

    Ok(())
}

// Helper function to toggle an option
fn toggle_option<F>(sessions: &Sessions, chat_id: ChatId, toggle_fn: F)
where
    F: FnOnce(&mut crate::telegram_bot::BuildConfig),
{
    let mut sessions_lock = sessions.lock().unwrap();
    if let Some(session) = sessions_lock.get_mut(&chat_id) {
        toggle_fn(&mut session.config);
    }
}
