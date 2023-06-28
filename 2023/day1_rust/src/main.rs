mod part_one;
mod part_two;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut calibration_value = 0;
    let mut calibration_value_2 = 0;
    for line in lines.iter() {
        if let Ok(value) = part_one::get_calibration_value(line) {
            calibration_value += value;
        }
        if let Ok(value) = part_two::get_calibration_value(line) {
            calibration_value_2 += value;
        }
    }

    println!("Calibration value is: {}", calibration_value);
    println!("Corrected calibration value is: {}", calibration_value_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part_one::get_calibration_value;
    use crate::part_two::get_calibration_value as get_calibration_value_v2;

    #[test] fn test_calibration_value() {
        assert_eq!(get_calibration_value("1abc2"), Ok(12));
        assert_eq!(get_calibration_value("pqr3stu8vwx"), Ok(38));
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), Ok(15));
        assert_eq!(get_calibration_value("treb7uchet"), Ok(77));
        assert_eq!(
            get_calibration_value("trebuchet"),
            Err("No digits found".to_string())
        );
    }

    #[test]
    fn test_calibration_value_v2() {
        assert_eq!(get_calibration_value_v2("two1nine "), Ok(29));
        assert_eq!(get_calibration_value_v2("eightwothree "), Ok(83));
        assert_eq!(get_calibration_value_v2("abcone2threexyz "), Ok(13));
        assert_eq!(get_calibration_value_v2("xtwone3four "), Ok(24));
        assert_eq!(get_calibration_value_v2("4nineeightseven2 "), Ok(42));
        assert_eq!(get_calibration_value_v2("zoneight234 "), Ok(14));
        assert_eq!(get_calibration_value_v2("7pqrstsixteen "), Ok(76));

        assert_eq!(get_calibration_value_v2("1one"), Ok(11));
        assert_eq!(get_calibration_value_v2("1one1"), Ok(11));
        assert_eq!(get_calibration_value_v2("one1"), Ok(11));
        assert_eq!(get_calibration_value_v2("aone1two"), Ok(12));

        assert_eq!(get_calibration_value_v2("42"), Ok(42));

        assert_eq!(get_calibration_value_v2("zero"), Ok(0));
        assert_eq!(get_calibration_value_v2("one"), Ok(11));
        assert_eq!(get_calibration_value_v2("two"), Ok(22));
        assert_eq!(get_calibration_value_v2("three"), Ok(33));
        assert_eq!(get_calibration_value_v2("four"), Ok(44));
        assert_eq!(get_calibration_value_v2("five"), Ok(55));
        assert_eq!(get_calibration_value_v2("six"), Ok(66));
        assert_eq!(get_calibration_value_v2("seven"), Ok(77));
        assert_eq!(get_calibration_value_v2("eight"), Ok(88));
        assert_eq!(get_calibration_value_v2("nine"), Ok(99));
        assert_eq!(get_calibration_value_v2(""), Err("No digits found".to_string()));

        assert_eq!(get_calibration_value_v2("92eightlsgrmpqtpptxrdfxthreemvlxfpsevenoneightdd"), Ok(98));
    }
}
