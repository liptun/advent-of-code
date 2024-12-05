pub struct WordSearch {
    text: Vec<String>,
    word_to_find: String,
}

impl WordSearch {
    pub fn new(word: &str) -> Self {
        Self {
            text: Vec::new(),
            word_to_find: word.to_string(),
        }
    }

    pub fn text_add_line(&mut self, line: &str) -> () {
        self.text.push(line.to_string());
    }

    pub fn count_word_occurences_in_line(line: &str, word: &str) -> u32 {
        line.matches(word).count() as u32
    }

    pub fn count_word_occurences_in_lines(lines: &Vec<String>, word: &str) -> u32 {
        let mut count = 0;
        for line in lines.iter() {
            count += WordSearch::count_word_occurences_in_line(line, word);
        }
        count
    }

    pub fn get_vertical_lines(lines: &Vec<String>) -> Option<Vec<String>> {
        let mut vertical_lines = Vec::new();

        let col_count = if let Some(line) = lines.get(0) {
            line.len()
        } else {
            return None;
        };

        for col in 0..col_count {
            let mut col_text = String::new();
            for line in 0..lines.len() {
                let char = lines[line].chars().nth(col).unwrap_or(' ');
                col_text.push(char);
            }
            vertical_lines.push(col_text);
        }

        Some(vertical_lines)
    }

    pub fn get_diagonal_lines(lines: &Vec<String>) -> Option<Vec<String>> {
        None
    }

    fn count_left_to_right(&self) -> u32 {
        WordSearch::count_word_occurences_in_lines(&self.text, &self.word_to_find)
    }

    fn count_right_to_left(&self) -> u32 {
        let word_reverse: String = self.word_to_find.chars().rev().collect();
        WordSearch::count_word_occurences_in_lines(&self.text, &word_reverse)
    }

    fn count_top_to_bottom(&self) -> u32 {
        if let Some(lines) = WordSearch::get_vertical_lines(&self.text) {
            WordSearch::count_word_occurences_in_lines(&lines, &self.word_to_find)
        } else {
            0
        }
    }

    fn count_bottom_to_top(&self) -> u32 {
        let word_reverse: String = self.word_to_find.chars().rev().collect();
        if let Some(lines) = WordSearch::get_vertical_lines(&self.text) {
            WordSearch::count_word_occurences_in_lines(&lines, &word_reverse)
        } else {
            0
        }
    }

    pub fn count_occurences(&self) -> u32 {
        self.count_left_to_right()
            + self.count_right_to_left()
            + self.count_top_to_bottom()
            + self.count_bottom_to_top()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_word_occurences_in_line() {
        assert_eq!(
            WordSearch::count_word_occurences_in_line("testxtestxtestx", "test"),
            3
        );
    }

    #[test]
    fn count_word_occurences_in_lines() {
        let lines: Vec<String> = vec![
            "xtestxtest".to_string(),
            "testtesttest".to_string(),
            "xtetest".to_string(),
        ];
        assert_eq!(
            WordSearch::count_word_occurences_in_lines(&lines, "test"),
            6
        );
    }

    #[test]
    fn get_vertical_lines_test() {
        let lines = vec!["123".to_string(), "456".to_string(), "789".to_string()];

        let vertical_lines =
            WordSearch::get_vertical_lines(&lines).expect("Should get vertical lines");

        assert_eq!(vertical_lines.get(0).expect("Should get first line"), "147");
    }

    #[test]
    fn get_diaglonal_lines_test() {
        let lines = vec!["123".to_string(), "456".to_string(), "789".to_string()];

        let vertical_lines =
            WordSearch::get_vertical_lines(&lines).expect("Should get vertical lines");

        assert_eq!(vertical_lines.get(0).expect("Should get first line"), "159");
    }

    #[test]
    fn count_occurences_test() {
        let mut word_search = WordSearch::new("XMAS");

        let text: Vec<&str> = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        for line in text {
            word_search.text_add_line(line);
        }
        assert_eq!(word_search.count_occurences(), 18);
    }
}
