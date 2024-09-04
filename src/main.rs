mod cards;
mod dictionary;

use crate::cards::{get_card, WordsLanguage};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match get_card(WordsLanguage::EN, 8) {
        Ok(words_en) => {
            println!("Card with english words:");
            for word in &words_en {
                println!("{}", word);
            }
        }
        Err(e) => eprintln!("Error getting english card: {}", e),
    }

    println!("");

    match get_card(WordsLanguage::RU, 8) {
        Ok(words_ru) => {
            println!("Card with russian words:");
            for word in &words_ru {
                println!("{}", word);
            }
        }
        Err(e) => eprintln!("Error getting russian card: {}", e),
    }

    println!("");

    match create_mixed_card() {
        Ok(words_en_ru) => {
            println!("Card with english/russian words:");
            for word in &words_en_ru {
                println!("{}", word);
            }
        }
        Err(e) => eprintln!("Error getting english/russian card: {}", e),
    }

    Ok(())
}

fn create_mixed_card() -> Result<Vec<&'static str>, Box<dyn Error>> {
    let mut mixed_card = Vec::new();

    match get_card(WordsLanguage::EN, 4) {
        Ok(words_en) => {
            for word in words_en {
                mixed_card.push(word);
            }
        }
        Err(e) => eprintln!("Error getting 4 random english words for mixed card: {}", e),
    }

    match get_card(WordsLanguage::RU, 4) {
        Ok(words_en) => {
            for word in words_en {
                mixed_card.push(word);
            }
        }
        Err(e) => eprintln!("Error getting 4 random russian words for mixed card: {}", e),
    }

    Ok(mixed_card)
}
