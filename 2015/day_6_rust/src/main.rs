use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use command::Command;
use lights::{Light, Lights};

mod command;
mod lights;
mod vector2_ext;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut lights = Lights::new();

    for command in reader
        .lines()
        .filter_map(Result::ok)
        .map(|c| Command::new(&c))
        .filter_map(Result::ok)
    {
        lights.exec(command);
    }

    let lights_lit = lights
        .grid
        .iter()
        .filter(|&x| matches!(x, Light::Lit))
        .count();
    println!("Lights lit {}", lights_lit);

    Ok(())
}
