mod card;
use card::Card;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut total_score = 0;

    for line in lines.iter() {
        if let Ok(card) = Card::new(line) {
            total_score += card.calculate_points();
        }
    }

    println!("total score: {}", total_score);

    // part 2

    let mut cards_count: [u32; 204] = [1; 204];

    for line in lines.iter() {
        if let Ok(card) = Card::new(line) {
            let card_count = cards_count[card.no as usize - 1];
            println!("card {} has {} matches and {} copies", card.no, card.get_matches().len(), card_count);
            for _ in 0..card_count {
                for i in card.no as usize..card.no as usize + card.get_matches().len() {
                    cards_count[i] += 1;
                }
            }
        }
    }

    let total_cards: u32 = cards_count.iter().sum();

    println!("Total cards {}", total_cards);

    Ok(())
}
