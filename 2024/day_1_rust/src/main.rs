use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in reader.lines().filter_map(Result::ok) {
        println!("{}", line);
        if let Some((a, b)) = parse_line_to_tuple(line) {
            left_list.push(a);
            right_list.push(b);
        }
    }

    left_list.sort();
    right_list.sort();

    let mut total_distance_between_lists: u32 = 0;

    for i in 0..left_list.len() {
        let a = left_list[i];
        let b = right_list[i];

        total_distance_between_lists += get_distance_between_numbers(a, b)
    }

    println!(
        "Total distance between lists is: {}",
        total_distance_between_lists
    );

    let mut similarity: u32 = 0;

    for a_index in 0..left_list.len() {
        let a = left_list[a_index];
        let mut occurences = 0;
        for b in right_list.iter() {
            if a == *b {
               occurences += 1; 
            }
        }
        similarity += a * occurences;
    }

    println!(
        "Similarity is: {}",
        similarity
    );

    Ok(())
}

fn get_distance_between_numbers(a: u32, b: u32) -> u32 {
    a.abs_diff(b)
}

fn parse_line_to_tuple(line: String) -> Option<(u32, u32)> {
    let a = line.get(..5);
    let b = line.get(8..);
    if let Some((a, b)) = a.zip(b) {
        let a_int: u32 = a.parse().unwrap_or(0);
        let b_int: u32 = b.parse().unwrap_or(0);
        return Some((a_int, b_int));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_distance_test() {
        assert_eq!(get_distance_between_numbers(1, 3), 2);
        assert_eq!(get_distance_between_numbers(2, 3), 1);
        assert_eq!(get_distance_between_numbers(3, 3), 0);
        assert_eq!(get_distance_between_numbers(3, 4), 1);
        assert_eq!(get_distance_between_numbers(3, 5), 2);
        assert_eq!(get_distance_between_numbers(4, 9), 5);
        assert_eq!(get_distance_between_numbers(3, 1), 2);
    }

    #[test]
    fn parse_line_test() {
        assert_eq!(
            parse_line_to_tuple("66845   37619".to_string()),
            Some((66845, 37619))
        );
    }
}
