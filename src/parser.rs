use crate::stopwords::stopwords;
use radix_trie::Trie;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use regex::Regex;
use std::borrow::BorrowMut;
use std::fs;

pub struct ParsedFile {
    pub doc_id: String,
    pub category: String,
    pub total_word_count: u32,
    pub counted_words: Trie<String, u32>,
}

pub fn parse() -> Result<Vec<ParsedFile>, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref WORD_REGEX: Regex = Regex::new("([a-z][a-z'\\-]+[a-z])").unwrap();
    }

    let res = walkdir::WalkDir::new("datasets/articles")
        .into_iter()
        // take only files and ignore directories
        // also crash on any error since it's supposed to run in parallel
        .filter(|entry| {
            entry
                .as_ref()
                .expect("WalkDir entry error")
                .metadata()
                .expect("WalkDir Metadata error")
                .is_file()
        })
        .par_bridge()
        .map(|entry| {
            let path = entry.as_ref().unwrap().path();
            let mut path_iter = path.iter();

            let category = path_iter
                .nth(2)
                .expect("bad path category")
                .to_str()
                .unwrap()
                .to_string();

            let doc_id = path_iter
                .next()
                .expect("bad path doc_id")
                .to_str()
                .unwrap()
                .to_string();

            drop(path_iter);

            let contents = fs::read_to_string(path);

            if contents.is_err() {
                return None;
            }

            let contents = contents.unwrap();

            let mut contents_chars = contents.chars();

            let mut prev_was_new_line = false;

            contents_chars
                .borrow_mut()
                .take_while(|c| {
                    if *c == '\n' {
                        if prev_was_new_line {
                            return false;
                        } else {
                            prev_was_new_line = true;
                            return true;
                        }
                    } else {
                        prev_was_new_line = false;
                        return true;
                    }
                })
                .for_each(|_| {});

            let body = contents_chars.collect::<String>().to_lowercase();

            let mut total_word_count = 0u32;
            let mut counted_words = Trie::<String, u32>::new();

            WORD_REGEX
                .captures_iter(body.as_str())
                .map(|caps| caps.get(1).map_or("", |m| m.as_str()))
                .filter(|word| !stopwords().contains(*word))
                .for_each(|word| {
                    total_word_count += 1;

                    if let Some(counter) = counted_words.get_mut(word) {
                        *counter += 1;
                    } else {
                        counted_words.insert(word.to_string(), 1);
                    }
                });

            return Some(ParsedFile {
                doc_id,
                category,
                total_word_count,
                counted_words,
            });
        })
        .filter(|maybe_result| maybe_result.is_some())
        .map(|maybe_result| maybe_result.unwrap())
        .collect::<Vec<ParsedFile>>();

    Ok(res)
}
