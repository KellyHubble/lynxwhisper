pub fn clean_text(text: &str) -> String {
    // Remove common Whisper tags
    // TODO: Make general for [**]
    // TODO: Consider handling `(**)`
    let cleaned = text
        .replace("[BLANK_AUDIO]", "")
        .replace("[NO_SPEECH]", "")
        .replace("[MUSIC]", "")
        .trim() // Drop leading/trailing whitespace
        .to_string();
    println!("Cleaned text: {}", cleaned);
    cleaned
}

// Placeholder for future task processing
pub fn process_tasks(_text: &str) -> String {
    // TODO: Add keyword/phrase detection (e.g., "email bro" -> send email)
    // For now, just pass through
    String::new()
}