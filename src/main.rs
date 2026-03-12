use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc::channel};

use arboard::Clipboard;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;

const FILE_PATH: &str = "/home/test/.steam/steam/steamapps/common/Path of Exile/logs/Client.txt";

/// Extracts the last string following a "+" in the content.
/// Example: "#Some-Character: Uber Maven +invite123" → "invite123"
fn extract_last_invitation(content: &str) -> Option<String> {
    let re = Regex::new(r"\+([^\s]+)").unwrap();

    re.captures_iter(content)
        .last()
        .map(|cap| cap[1].to_string())
}

fn main() -> notify::Result<()> {
    let file = PathBuf::from(FILE_PATH)
        .canonicalize()
        .expect("Failed to resolve file path");
    let last_value = Arc::new(Mutex::new(String::new()));

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&file, RecursiveMode::NonRecursive)?;

    println!("Watching {:?}", file);

    let mut clipboard = Clipboard::new().unwrap();

    for event in rx.into_iter().flatten() {
        if matches!(event.kind, EventKind::Modify(_))
            && let Ok(content) = fs::read_to_string(&file)
            && let Some(value) = extract_last_invitation(&content)
        {
            let mut stored = last_value.lock().unwrap();

            if *stored != value {
                *stored = value.clone();
                clipboard.set_text(format!("#+{}", value)).unwrap();
                println!("New invitation detected: {}", value);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_last_invitation() {
        let content = "#Some-Character: Uber Maven +invite123 +anotherInvite";
        let result = extract_last_invitation(content);
        assert_eq!(result, Some("anotherInvite".to_string()));
    }

    #[test]
    fn test_extract_no_invitation() {
        let content = "#Some-Character: Uber Maven";
        let result = extract_last_invitation(content);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_single_invitation() {
        let content = "#Some-Character: Uber Maven +invite123";
        let result = extract_last_invitation(content);
        assert_eq!(result, Some("invite123".to_string()));
    }
}
