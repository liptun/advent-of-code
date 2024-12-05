mod word_search;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use word_search::WordSearch;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let word_to_find = "XMAS";
    let mut word_search = WordSearch::new(word_to_find);

    for line in reader.lines().filter_map(Result::ok) {
        word_search.text_add_line(&line);
    }


    println!(
        "Word \"{}\" occurs {} times",
        word_to_find,
        word_search.count_occurences()
    );

    Ok(())
}
