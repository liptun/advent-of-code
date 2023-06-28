pub fn get_calibration_value(input_string: &str) -> Result<i32, String> {
    let digits: Vec<char> = input_string.chars().filter(|s| s.is_numeric()).collect();

    let first_digit = digits
        .first()
        .ok_or_else(|| "No digits found".to_string())?;
    let last_digit = digits.last().ok_or_else(|| "No digits found".to_string())?;

    let combined_str = format!("{}{}", first_digit, last_digit);

    combined_str.parse::<i32>().map_err(|e| e.to_string())
}
