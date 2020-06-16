use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

lazy_static! {
    static ref STOP_WORDS: HashSet<String> = {
        let mut stopwords = HashSet::<String>::new();

        let file = File::open("datasets/stopwords.txt").expect("error loading stopwords");
        io::BufReader::new(file).lines().for_each(|stopword| {
            let stopword = stopword.expect("error reading given stopword");
            stopwords.insert(stopword);
        });

        stopwords
    };
}

pub fn stopwords() -> &'static HashSet<String> {
    &STOP_WORDS
}
