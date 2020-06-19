#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate radix_trie;
extern crate rayon;
extern crate regex;
extern crate walkdir;

mod counted_word;
mod parser;
mod stopwords;

use crate::counted_word::CountedWord;
use crate::radix_trie::TrieCommon;
use itertools::join;
use parser::parse;
use radix_trie::Trie;
use rayon::prelude::*;

const NUMBER_OF_SELECTED_WORDS: usize = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_files = parse()?;

    let mut global_words = Trie::<String, u32>::new();

    for file in parsed_files.iter() {
        for (word, count) in file.counted_words.iter() {
            if let Some(counter) = global_words.get_mut(word) {
                *counter += count;
            } else {
                global_words.insert(word.clone(), *count);
            }
        }
    }

    let mut most_common_words = global_words
        .iter()
        // filter out some nice
        .filter(|(_word, count)| **count > 10)
        .map(|(word, count)| CountedWord {
            word,
            count: *count,
        })
        .collect::<Vec<_>>();

    most_common_words.as_mut_slice().par_sort_unstable();

    let selected_words_iter = most_common_words
        .iter()
        .skip(most_common_words.len() - NUMBER_OF_SELECTED_WORDS);

    assert_eq!(
        NUMBER_OF_SELECTED_WORDS,
        selected_words_iter.clone().count()
    );

    println!("@RELATION words\n");

    for selected_word in selected_words_iter.clone() {
        println!("@ATTRIBUTE \"{}\" real", selected_word.word);
    }

    let mut categories = Trie::<String, ()>::new();

    for file in parsed_files.iter() {
        categories.insert(file.category.clone(), ());
    }

    let categories = categories
        .iter()
        .map(|(category, _)| format!("\"{}\"", category));

    println!("@ATTRIBUTE CLASS {{{}}}\n\n@DATA", join(categories, ","));

    let selected_words = selected_words_iter.map(|sw| sw.word).collect::<Vec<_>>();

    for file in parsed_files {
        for &selected_word in selected_words.iter() {
            let frequency = if let Some(count) = file.counted_words.get(selected_word) {
                *count as f32 / file.total_word_count
            } else {
                0f32
            };

            print!("{},", frequency);
        }

        println!("\"{}\"", file.category)
    }

    Ok(())
}
