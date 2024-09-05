use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fmt;

use crate::dictionary::{en, ru};

pub enum WordsLanguage {
    EN,
    RU,
}

pub fn get_card(
    source: WordsLanguage,
    quantity: usize,
) -> Result<Vec<&'static str>, Box<dyn Error>> {
    let dict = match source {
        WordsLanguage::EN => en::DICT,
        WordsLanguage::RU => ru::DICT,
    };

    get_words(dict, quantity)
}

fn get_words<'a>(source: &'a [&str], quantity: usize) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let source_len = source.len();
    if quantity > source_len {
        return Err(Box::new(ErrorWordsQuantity {
            requested: quantity,
            available: source_len,
        }));
    }

    let mut rng = thread_rng();

    let indices: Vec<usize> = (0..source_len).collect();
    let mut rng_indices = indices.clone();
    rng_indices.shuffle(&mut rng);

    let result = rng_indices
        .into_iter()
        .take(quantity)
        .map(|i| source[i])
        .collect();

    Ok(result)
}

#[derive(Debug)]
struct ErrorWordsQuantity {
    requested: usize,
    available: usize,
}

impl fmt::Display for ErrorWordsQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Requested quantity {} exceeds the number of available words {}",
            self.requested, self.available
        )
    }
}

impl Error for ErrorWordsQuantity {}

#[cfg(test)]
mod tests;
