use std::process::Command;

pub fn check_for_updates() -> Option<String> {
    // Check if git is available
    let git_check = Command::new("git")
        .arg("--version")
        .output();
    
    if git_check.is_err() {
        return None;
    }
    
    // Fetch latest from remote
    let _ = Command::new("git")
        .args(&["fetch", "origin"])
        .output();
    
    // Check if branch is behind
    let status = Command::new("git")
        .args(&["status", "-uno"])
        .output();
    
    if let Ok(output) = status {
        let status_text = String::from_utf8_lossy(&output.stdout);
        
        if status_text.contains("Your branch is behind") {
            // Get commit count
            let commits = Command::new("git")
                .args(&["rev-list", "--count", "HEAD..origin/main"])
                .output();
            
            if let Ok(commit_output) = commits {
                let count = String::from_utf8_lossy(&commit_output.stdout).trim().to_string();
                return Some(format!("{} new commit(s)", count));
            }
            
            return Some("Updates available".to_string());
        }
    }
    
    None
}

pub fn get_latest_changes() -> String {
    let log = Command::new("git")
        .args(&["log", "HEAD..origin/main", "--oneline", "--max-count=5"])
        .output();
    
    if let Ok(output) = log {
        let changes = String::from_utf8_lossy(&output.stdout);
        if !changes.is_empty() {
            return changes.to_string();
        }
    }
    
    "Unable to fetch changes".to_string()
}

pub fn perform_update() -> Result<String, String> {
    let pull = Command::new("git")
        .args(&["pull", "origin", "main"])
        .output();
    
    match pull {
        Ok(output) => {
            if output.status.success() {
                Ok("Update successful! Please restart the bot.".to_string())
            } else {
                Err("Update failed. Check git status.".to_string())
            }
        }
        Err(_) => Err("Git not available".to_string()),
    }
}
