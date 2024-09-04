use super::*;
use crate::dictionary;

#[test]
fn test_get_card_english_success() {
    match get_card(WordsLanguage::EN, 8) {
        Ok(words) => {
            assert_eq!(words.len(), 8);

            for word in &words {
                assert!(dictionary::en::DICT.contains(word));
            }
        }
        Err(e) => panic!("Error getting eight random english words: {}", e),
    }
}

#[test]
fn test_get_card_russian_success() {
    match get_card(WordsLanguage::RU, 8) {
        Ok(words) => {
            assert_eq!(words.len(), 8);

            for word in &words {
                assert!(dictionary::ru::DICT.contains(word));
            }
        }
        Err(e) => panic!("Error getting eight random russian words: {}", e),
    }
}
