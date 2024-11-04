use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

mod nice_string;
use nice_string::*;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut nice_string_count = 0;
    for s in lines {
        if let StringType::Nice = check_string(&s) {
            nice_string_count += 1;
        }
    }

    println!("Nice string count {}", nice_string_count);

    Ok(())
}

