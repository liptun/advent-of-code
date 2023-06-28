use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn is_valid_part_coords(
    shematic: &Vec<String>,
    line_no: usize,
    start: usize,
    length: usize,
) -> bool {
    for line_offset in 0..=2 {
        let line_index = (line_no + line_offset).checked_sub(1).unwrap_or(0);

        if let Some(curr_line) = shematic.get(line_index) {
            let slice_start = start.checked_sub(1).unwrap_or(0);
            let slice_end = min(start + length + 1, curr_line.len());
            let slice = &curr_line[slice_start..slice_end];

            for c in slice.chars().into_iter() {
                if c != '.' && !c.is_numeric() {
                    return true;
                }
            }
        }
    }
    false
}

fn get_valid_parts(input: Vec<String>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for (line_no, line) in input.iter().enumerate() {
        let mut line_chars = line.chars();
        let mut char_index = 0;
        let mut pocket: Vec<char> = Vec::new();
        while char_index <= line.len() {
            let curr_char: char = line_chars.next().unwrap_or('.');
            if curr_char.is_numeric() {
                pocket.push(curr_char);
            } else {
                if pocket.len() > 0 {
                    let start = char_index.checked_sub(pocket.len()).unwrap_or(0);
                    if is_valid_part_coords(&input, line_no, start, pocket.len()) {
                        let part_no: u32 = pocket
                            .iter()
                            .fold(0, |acc, &digit| acc * 10 + digit.to_digit(10).unwrap_or(0));
                        result.push(part_no);
                    }

                    pocket.clear();
                }
            }
            char_index += 1;
        }
    }
    result
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let parts = get_valid_parts(lines);
    let sum: u32 = parts.iter().sum();
    println!("sum of parts numbers is {}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{get_valid_parts, is_valid_part_coords};

    #[test]
    fn test_is_valid_part_coords() {
        let test = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*.....@",
            ".....+.666",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let test_input: Vec<String> = test.iter().map(|s| s.to_string()).collect();

        assert_eq!(is_valid_part_coords(&test_input, 0, 0, 3), true);
        assert_eq!(is_valid_part_coords(&test_input, 0, 5, 3), false);
        assert_eq!(is_valid_part_coords(&test_input, 2, 2, 2), true);
        assert_eq!(is_valid_part_coords(&test_input, 5, 7, 3), true);
    }

    #[test]
    fn test_get_valid_parts() {
        let test = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let test_input: Vec<String> = test.iter().map(|s| s.to_string()).collect();

        let result: u32 = get_valid_parts(test_input).iter().sum();
        assert_eq!(result, 4361);
    }
}
