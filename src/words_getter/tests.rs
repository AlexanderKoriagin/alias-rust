use super::*;
use crate::dictionary;

#[test]
fn test_get_eight_random_english_words_success() {
    match get_random_words(WordsLanguage::EN, 8) {
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
fn test_get_eight_random_russian_words_success() {
    match get_random_words(WordsLanguage::RU, 8) {
        Ok(words) => {
            assert_eq!(words.len(), 8);

            for word in &words {
                assert!(dictionary::ru::DICT.contains(word));
            }
        }
        Err(e) => panic!("Error getting eight random russian words: {}", e),
    }
}
