use radix_trie::Trie;
use std::fs::File;
use std::io::{self, BufRead};

lazy_static! {
    static ref STOP_WORDS: Trie<String, ()> = {
        let mut stopwords = Trie::<String, ()>::new();

        let file =
            File::open("datasets/stopwords.txt").expect("couldn't open datasets/stopwords.txt");

        io::BufReader::new(file).lines().for_each(|stopword| {
            let stopword = stopword.expect("error reading given stopword");
            stopwords.insert(stopword, ());
        });

        stopwords
    };
}

pub fn is_stopword(word: &str) -> bool {
    STOP_WORDS.get(word).is_some()
}
