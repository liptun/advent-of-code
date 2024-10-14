pub fn get_calibration_value(input_string: &str) -> Result<i32, String> {
    let mut digits: Vec<char> = Vec::new();
    let digits_dictionary = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut index: usize = 0;

    while index < input_string.len() {
        let current_string = &input_string[index..];
        let first_char = current_string.chars().next().unwrap();
        if first_char.is_numeric() {
            digits.push(first_char);
            index += 1;
        } else {
            for (i, d) in digits_dictionary.iter().enumerate() {
                if current_string.starts_with(d) {
                    let iu8 = i as u8;
                    if let Some(char) = char::from_digit(iu8.into(), 10) {
                        digits.push(char);
                    }
                    break;
                }
            }
            index += 1;
        }
    }

    let first_digit = digits
        .first()
        .ok_or_else(|| "No digits found".to_string())?;
    let last_digit = digits.last().ok_or_else(|| "No digits found".to_string())?;

    let combined_str = format!("{}{}", first_digit, last_digit);

    combined_str.parse::<i32>().map_err(|e| e.to_string())
}
