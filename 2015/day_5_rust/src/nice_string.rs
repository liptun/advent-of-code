use std::collections::HashMap;

pub fn is_nice_string(s: &str) -> bool {
    is_contains_at_least_three_vowels(s)
}

fn get_vowels(s: &str) -> HashMap<char, u8> {
    let mut vowels: HashMap<char, u8> = HashMap::new();

    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let v = vowels.entry(c).or_insert(0);
                *v += 1;
            }
            _ => {}
        }
    }

    vowels
}

fn is_contains_at_least_three_vowels(s: &str) -> bool {
    get_vowels(s).values().sum::<u8>() >= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vowels() {
        assert_eq!(get_vowels("aei").len(), 3);
        assert_eq!(get_vowels("aaa").values().sum::<u8>(), 3);
        assert_eq!(get_vowels("aaaa").values().sum::<u8>(), 4);
    }

    #[test]
    fn test_contains_three_vowels() {
        assert_eq!(is_contains_at_least_three_vowels("aei"), true);
        assert_eq!(is_contains_at_least_three_vowels("xazegov"), true);
        assert_eq!(is_contains_at_least_three_vowels("aeiouaeiouaeiou"), true);
    }

    //#[test]
    fn test_is_nice_string() {
        assert_eq!(is_nice_string("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_string("aaa"), true);

        assert_eq!(is_nice_string("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_string("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_string("haegwjzuvuyypxyu"), false);
    }
}
