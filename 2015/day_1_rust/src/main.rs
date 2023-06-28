use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut floor = 0;
    let mut basement_step = 0;

    for (index, c) in lines[0].chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 && basement_step == 0 {
            basement_step = index + 1;
        }
        println!("{} {}", index, floor)
    }
    println!("final floor {}", floor);
    println!("entered basement on {} character", basement_step);

    Ok(())
}
