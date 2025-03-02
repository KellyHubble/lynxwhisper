use rdev::{listen, EventType, Key};
use tokio::sync::mpsc::Sender;
use std::collections::HashSet;
use crate::config::Hotkeys;

pub fn start_hotkey_listener(tx: Sender<String>, hotkeys: Hotkeys) {
    std::thread::spawn(move || {
        println!("Hotkey listener started");
        let mut pressed_keys = HashSet::new();
        listen(move |event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    pressed_keys.insert(key);
                    let combo = build_combo(&pressed_keys, &hotkeys);
                    if !combo.is_empty() {
                        println!("Sending combo: {}", combo);
                        tx.try_send(combo).unwrap_or_else(|e| println!("Send failed: {}", e));
                    }
                }
                EventType::KeyRelease(key) => {
                    pressed_keys.remove(&key);
                }
                _ => {}
            }
        }).unwrap_or_else(|e| println!("Listen failed: {:?}", e));
    });
}

fn build_combo(keys: &HashSet<Key>, hotkeys: &Hotkeys) -> String {
    let mut combo = String::new();
    let mut has_modifier = false;
    if keys.contains(&Key::ControlLeft) || keys.contains(&Key::ControlRight) {
        combo.push_str("ControlRight+"); // Right Ctrl for clarity
        has_modifier = true;
    }
    if keys.contains(&Key::ShiftLeft) || keys.contains(&Key::ShiftRight) {
        combo.push_str("ShiftRight+"); // Right Shift
        has_modifier = true;
    }
    let final_keys = [
        (Key::Num1, &hotkeys.manual_start),
        (Key::Num1, &hotkeys.manual_stop),
        (Key::Num3, &hotkeys.automatic_toggle),
    ];
    for (key, config_key) in final_keys.iter() {
        if keys.contains(key) && has_modifier {
            let mapped = map_key_to_string(key);
            combo.push_str(&mapped);
            if matches_hotkey(&combo, config_key) {
                return combo;
            }
        }
    }
    String::new()
}

fn map_key_to_string(key: &Key) -> String {
    match key {
        Key::Num1 => "Key1".to_string(),
        Key::Num2 => "Key2".to_string(),
        Key::Num3 => "Key3".to_string(),
        _ => format!("{:?}", key),
    }
}

fn parse_hotkey(hotkey: &str) -> Vec<&str> {
    hotkey.split('+').map(|s| s.trim()).collect()
}

pub fn matches_hotkey(event: &str, hotkey: &str) -> bool {
    let hotkey_parts = parse_hotkey(hotkey);
    let mapped_hotkey: String = hotkey_parts.iter().map(|&part| match part {
        "Ctrl" => "ControlRight",
        "Shift" => "ShiftRight",
        "1" => "Key1",
        "2" => "Key2",
        "3" => "Key3",
        _ => part,
    }).collect::<Vec<&str>>().join("+");
    let is_match = event == mapped_hotkey;
    println!("Event: {}, Mapped hotkey: {}, Match: {}", event, mapped_hotkey, is_match);
    is_match
}