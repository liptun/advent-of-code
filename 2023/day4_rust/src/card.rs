#[derive(Debug, PartialEq)]
pub enum CardCreationError {
    NoNameSeparator,
    NoNumbersSeparator,
    NoNumberInName,
    NoWinningNumbers,
    NoNumbers,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
    pub no: u32,
    pub winning: Vec<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    pub fn new(s: &str) -> Result<Self, CardCreationError> {
        let name_content: Vec<&str> = s.split(":").collect();
        if name_content.len() != 2 {
            return Err(CardCreationError::NoNameSeparator);
        }

        let name = name_content[0].to_string();

        let name_number: Vec<&str> = name_content[0].split_whitespace().collect();

        let no: u32 = match name_number[1].parse() {
            Ok(n) => n,
            Err(_) => return Err(CardCreationError::NoNumberInName),
        };

        let content: Vec<&str> = name_content[1].split('|').collect();
        if content.len() != 2 {
            return Err(CardCreationError::NoNumbersSeparator);
        }
        let winning: Vec<u32> = content[0]
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        if winning.len() == 0 {
            return Err(CardCreationError::NoWinningNumbers);
        }

        let numbers: Vec<u32> = content[1]
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        if numbers.len() == 0 {
            return Err(CardCreationError::NoNumbers);
        }

        Ok(Self {
            name,
            no,
            winning,
            numbers,
        })
    }

    pub fn get_matches(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .cloned()
            .collect()
    }

    pub fn calculate_points(&self) -> u32 {
        let matches = self.get_matches().len();
        let mut score = 0;
        if matches > 0 {
            score += 1;
        }
        if matches > 1 {
            for _ in 1..matches {
                score = score * 2;
            }
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_parsing_error() {
        assert!(matches!(
            Card::new(""),
            Err(CardCreationError::NoNameSeparator)
        ));
        assert!(matches!(
            Card::new("Card 1 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Err(CardCreationError::NoNameSeparator)
        ));
        assert!(matches!(
            Card::new("Card 1:"),
            Err(CardCreationError::NoNumbersSeparator)
        ));
        assert!(matches!(
            Card::new("Card 1: 41 48 83 86 17 83 86  6 31 17  9 48 53"),
            Err(CardCreationError::NoNumbersSeparator)
        ));
        assert!(matches!(
            Card::new("Card 1: |"),
            Err(CardCreationError::NoWinningNumbers)
        ));
        assert!(matches!(
            Card::new("Card 1: 23 |"),
            Err(CardCreationError::NoNumbers)
        ));
    }

    #[test]
    fn test_card_parsing() {
        let card = Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
            .expect("Should create Card");
        assert_eq!(card.name, "Card 1".to_string());
        assert_eq!(card.no, 1);
        assert_eq!(card.winning.len(), 5);
        assert_eq!(card.numbers.len(), 8);
        assert_eq!(card.winning, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);

        let card = Card::new("Card 22: 17 | 83").expect("Should create Card");
        assert_eq!(card.name, "Card 22".to_string());
        assert_eq!(card.no, 22);
    }

    #[test]
    fn test_card_matches() {
        let card1 = Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        let card2 = Card::new("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").unwrap();
        let card3 = Card::new("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").unwrap();
        let card4 = Card::new("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").unwrap();
        let card5 = Card::new("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").unwrap();
        let card6 = Card::new("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap();

        assert_eq!(card1.get_matches().len(), 4);
        assert_eq!(card2.get_matches().len(), 2);
        assert_eq!(card3.get_matches().len(), 2);
        assert_eq!(card4.get_matches().len(), 1);
        assert_eq!(card5.get_matches().len(), 0);
        assert_eq!(card6.get_matches().len(), 0);
    }

    #[test]
    fn test_card_points() {
        let card1 = Card::new("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        let card2 = Card::new("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").unwrap();
        let card3 = Card::new("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").unwrap();
        let card4 = Card::new("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").unwrap();
        let card5 = Card::new("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").unwrap();
        let card6 = Card::new("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap();

        assert_eq!(card1.calculate_points(), 8);
        assert_eq!(card2.calculate_points(), 2);
        assert_eq!(card3.calculate_points(), 2);
        assert_eq!(card4.calculate_points(), 1);
        assert_eq!(card5.calculate_points(), 0);
        assert_eq!(card6.calculate_points(), 0);
    }
}
