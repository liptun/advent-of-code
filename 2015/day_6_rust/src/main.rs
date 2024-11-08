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

fn scale_brightness(value: u32, max_brightness: u32) -> u8 {
    let scaled = (value as f32 / max_brightness as f32 * 255.0).round() as u8;
    scaled
}

fn lights_to_image(lights: &Lights, filename: &str) {
    let mut img: RgbImage = RgbImage::new(lights.size as u32, lights.size as u32);

    let max_brightness = *lights.grid.iter().max().unwrap();

    for (index, light) in lights.grid.iter().enumerate() {
        let pos = lights.index_to_vector(&index);

        let brightness = scale_brightness(*light, max_brightness);

        img.put_pixel(
            pos.x as u32,
            pos.y as u32,
            Rgb([brightness, brightness, brightness]),
        );
    }

    img.save(filename).unwrap();
}

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

    lights_to_image(&lights, "part1.png");
    lights_to_image(&lights_pt2, "part2.png");

    Ok(())
}
