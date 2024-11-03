use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

mod present_box;
use present_box::*;



fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut total_wrapping_paper_area = 0;
    let mut total_ribbon_length = 0;

    for s in lines {
        if let Ok(present_box) = PresentBox::new(&s) {
            total_wrapping_paper_area += present_box.get_surface_area_extra();
            total_ribbon_length += present_box.get_ribbon_lenght();
        }
    }

    println!("Total area of wrapping paper {} sq ft", total_wrapping_paper_area);
    println!("Total ribbon length {} ft", total_ribbon_length);
    Ok(())
}
