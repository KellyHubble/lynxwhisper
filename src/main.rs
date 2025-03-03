mod config;
mod audio;
mod hotkeys;
mod transcription;
mod typing;
mod postprocess;

use tokio::sync::mpsc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let config = config::load_config("config.toml").expect("Config loading failed");
    let (audio_tx, mut audio_rx) = mpsc::channel::<Vec<i16>>(100);
    let _audio_stream = audio::setup_audio_input(audio_tx);
    let (hotkey_tx, mut hotkey_rx) = mpsc::channel::<String>(100);
    hotkeys::start_hotkey_listener(hotkey_tx, config.hotkeys.clone());
    let transcriber = transcription::TranscriptionEngine::new(&config.model.path.to_string_lossy());
    let mut recording = false;
    let mut automatic_active = config.mode == "automatic";
    let mut audio_buffer = Vec::new();
    let mut silence_count = 0;
    // const SILENCE_THRESHOLD: i16 = 100; // Even lower for sensitivity
    const SILENCE_THRESHOLD: i16 = 3000; // for loud environments
    const SILENCE_CHUNKS: u32 = 10; // 

    loop {
        tokio::select! {
            Some(audio_data) = audio_rx.recv() => {
                if recording || (automatic_active && !audio_buffer.is_empty()) {
                    audio_buffer.extend(&audio_data);
                    println!("Recording, buffer size: {}", audio_buffer.len());
                    let max_amplitude = audio_data.iter().map(|&x| x.abs()).max().unwrap_or(0);
                    println!("Max amplitude: {}", max_amplitude);
                    if max_amplitude < SILENCE_THRESHOLD && automatic_active {
                        silence_count += 1;
                        println!("Silence detected, count: {}", silence_count);
                        if silence_count >= SILENCE_CHUNKS {
                            println!("Silence timeout, processing...");
                            recording = false;
                            if !audio_buffer.is_empty() {
                                let raw_text = transcriber.transcribe(&audio_buffer);
                                let text = postprocess::clean_text(&raw_text); 
                                println!("Text: {}", text);
                                if !text.is_empty() {
                                    typing::type_text(&text);
                                } else {
                                    println!("No text");
                                }
                                audio_buffer.clear();
                            }
                            silence_count = 0;
                        }
                    } else if max_amplitude > SILENCE_THRESHOLD * 2 { //Speech detected
                        silence_count = 0; // Reset silence counter
                    }
                }
            }
            Some(hotkey) = hotkey_rx.recv() => {
                println!("Hotkey: {}", hotkey);
                if hotkeys::matches_hotkey(&hotkey, &config.hotkeys.automatic_toggle) {
                    // Toggle automatic mode
                    automatic_active = !automatic_active;
                    println!("Auto toggle (Ctrl+Shift+3): {}", automatic_active);
                    if !automatic_active {
                        recording = false;
                        audio_buffer.clear();
                    }
                } else if hotkeys::matches_hotkey(&hotkey, &config.hotkeys.manual_stop) && recording {
                    // Stop recording manually
                    recording = false;
                    println!("Stop recording (Ctrl+Shift+2), buffer size: {}", audio_buffer.len());
                    if !audio_buffer.is_empty() {
                        let raw_text = transcriber.transcribe(&audio_buffer);
                        let text = postprocess::clean_text(&raw_text);
                        println!("Text: {}", text);
                        if !text.is_empty() {
                            typing::type_text(&text);
                        } else {
                            println!("No text");
                        }
                        audio_buffer.clear();
                    }
                } else if hotkeys::matches_hotkey(&hotkey, &config.hotkeys.manual_start) && !recording {
                    // Stop recording manually
                    recording = true;
                    audio_buffer.clear();
                    println!("Start recording (Ctrl+Shift+1) in auto mode");
                } 
            }
            _ = sleep(Duration::from_secs(config.automatic.chunk_interval)) => {
                if automatic_active && !recording && !audio_buffer.is_empty() {
                    println!("Auto transcribe (manual), buffer size: {}", audio_buffer.len());
                    let raw_text = transcriber.transcribe(&audio_buffer);
                    let text = postprocess::clean_text(&raw_text);
                    println!("Auto text: {}", text);
                    if !text.is_empty() {
                        typing::type_text(&text);
                    }
                    audio_buffer.clear();
                }
            }
        }
    }
}