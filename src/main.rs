mod cards;
mod dictionary;

use crate::cards::{get_random_words, WordsLanguage};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match get_random_words(WordsLanguage::EN, 8) {
        Ok(words_en) => {
            println!("Eight random English words:");
            for word in &words_en {
                println!("{}", word);
            }
        }
        Err(e) => eprintln!("Error getting eight random english words: {}", e),
    }

    match get_random_words(WordsLanguage::RU, 8) {
        Ok(words_ru) => {
            println!("Eight random English words:");
            for word in &words_ru {
                println!("{}", word);
            }
        }
        Err(e) => eprintln!("Error getting eight random russian words: {}", e),
    }

    Ok(())
}
