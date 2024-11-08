use image::{Rgb, RgbImage};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use command::Command;
use lights::Lights;

mod command;
mod lights;
mod vector2_ext;

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let commands: Vec<Command> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|c| Command::new(&c))
        .filter_map(Result::ok)
        .collect();

    // Part 1
    let mut lights = Lights::new();
    for command in commands.iter() {
        lights.exec(command);
    }
    let lights_lit = lights.grid.iter().filter(|&x| matches!(x, 1)).count();
    println!("Lights lit {}", lights_lit);

    // Part 2
    let mut lights_pt2 = Lights::new();
    for command in commands.iter() {
        lights_pt2.exec_pt2(command);
    }
    let total_brightness: u32 = lights_pt2.grid.iter().sum();
    println!("Total brightness {}", total_brightness);

    let mut img: RgbImage = RgbImage::new(1000, 1000);

    for (index, light) in lights.grid.iter().enumerate() {
        let pos = lights.index_to_vector(&index);

        let color = if *light == 1 {
            Rgb([255, 255, 255])
        } else {
            Rgb([0, 0, 0])
        };
        img.put_pixel(pos.x as u32, pos.y as u32, color);
    }

    img.save("part1.png").unwrap();

    Ok(())
}
