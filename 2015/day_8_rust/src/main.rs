use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn string_char_count(s: &str) -> usize {
    clean_string(s).len()
}

fn clean_string(s: &str) -> String {
    let trim = &s[1..s.len() - 1];
    let hex = Regex::new(r"\\x([0-9A-Fa-f]{2})").unwrap();
    let escape = Regex::new(r"\\(.)").unwrap();

    let hex_converted = hex.replace_all(trim, |caps: &regex::Captures| {
        let hex = &caps[1];
        let value = u8::from_str_radix(hex, 16).unwrap();
        char::from(value).to_string()
    });

    let final_converted = escape.replace_all(&hex_converted, |caps: &regex::Captures| {
        let escaped_char = &caps[1];
        escaped_char.to_string()
    });

    final_converted.to_string()
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_char_count = 0;
    for line in reader.lines().filter_map(Result::ok) {
        println!(
            "[{}]({}) -> [{}]({})",
            line,
            line.len(),
            clean_string(&line),
            string_char_count(&line)
        );
        total_char_count += line.len() - string_char_count(&line);
    }

    println!("Total chars {}", total_char_count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_string() {
        assert_eq!(clean_string("\"\\x27\""), "'");
        assert_eq!(clean_string("\"\\x2A\""), "*");
        assert_eq!(clean_string("\"\\x3D\""), "=");
    }

    #[test]
    fn test_string_char_count() {
        assert_eq!(string_char_count("\"\""), 0);
        assert_eq!(string_char_count("\"abc\""), 3);
        assert_eq!(string_char_count("\"aaa\\\"aaa\""), 7);
        assert_eq!(string_char_count("\"\x27\""), 1);
        assert_eq!(
            string_char_count("\"\\\"fxdnmvnftxwesmvvq\\\"sjnf\\xaabpg\\\"iary\""),
            33
        );
    }
}

