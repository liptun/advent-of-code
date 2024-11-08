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
        .filter(|&x| matches!(x, 1))
        .count();
    println!("Lights lit {}", lights_lit);

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
