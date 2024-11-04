use std::collections::HashMap;

#[derive(Debug)]
pub enum StringType {
    Nice,
    Naughty,
}

pub fn check_string(s: &str) -> StringType {
    if is_contains_at_least_three_vowels(s)
        && is_contains_at_least_one_doublet(s)
        && is_not_contains_forbidden_strings(s)
    {
        return StringType::Nice;
    }
    return StringType::Naughty;
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

fn get_doublets(s: &str) -> HashMap<String, u8> {
    let mut doublets: HashMap<String, u8> = HashMap::new();

    let chars: Vec<char> = s.chars().collect();

    let mut i: usize = 1;
    let mut prev: char = chars[0];

    while i < chars.len() {
        if chars[i] == prev {
            let key = prev.to_string().repeat(2);
            let d = doublets.entry(key).or_insert(0);
            *d += 1;
            i += 1;
        }

        prev = if let Some(c) = chars.get(i) {
            *c
        } else {
            break;
        };
        i += 1;
    }

    doublets
}

fn is_contains_at_least_one_doublet(s: &str) -> bool {
    get_doublets(s).values().sum::<u8>() >= 1
}

fn is_not_contains_forbidden_strings(s: &str) -> bool {
    let forbidden_strings: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
    for f in forbidden_strings.iter() {
        if s.contains(f) {
            return false;
        }
    }
    true
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

    #[test]
    fn test_get_doublets() {
        assert_eq!(get_doublets("aaeecc").len(), 3);
    }

    #[test]
    fn test_contains_one_doublet() {
        assert_eq!(is_contains_at_least_three_vowels("aei"), true);
    }

    #[test]
    fn test_contains_forbidden_strings() {
        assert_eq!(is_not_contains_forbidden_strings("abc"), false);
        assert_eq!(is_not_contains_forbidden_strings("cdd"), false);
        assert_eq!(is_not_contains_forbidden_strings("aad"), true);
    }

    #[test]
    fn test_check_string() {
        assert!(matches!(check_string("ugknbfddgicrmopn"), StringType::Nice));
        assert!(matches!(check_string("aaa"), StringType::Nice));
        assert!(matches!(check_string("jchzalrnumimnmhp"), StringType::Naughty));
        assert!(matches!(check_string("haegwjzuvuyypxyu"), StringType::Naughty));
        assert!(matches!(check_string("haegwjzuvuyypxyu"), StringType::Naughty));
    }
}
