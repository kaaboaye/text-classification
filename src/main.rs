#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate radix_trie;
extern crate rayon;
extern crate regex;
extern crate walkdir;

mod parser;
mod stopwords;

use crate::radix_trie::TrieCommon;
use parser::parse;
use radix_trie::Trie;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_files = parse()?;

    let mut total_count = 0u32;
    let mut global_words = Trie::<String, u32>::new();

    for file in parsed_files.iter() {
        total_count += file.total_word_count;

        for (word, count) in file.counted_words.iter() {
            if let Some(counter) = global_words.get_mut(word) {
                *counter += count;
            } else {
                global_words.insert(word.clone(), *count);
            }
        }
    }

    dbg!(total_count);

    Ok(())
}
