use enigo::{Enigo, KeyboardControllable};

pub fn type_text(text: &str) {
    let mut enigo = Enigo::new();
    println!("Typing: {}", text);
    enigo.key_sequence(text);  // Now works with trait in scope
}