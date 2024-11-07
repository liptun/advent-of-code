mod santa_route;
use std::fs;
use std::io::Error;
use utils::vector2::*;
use santa_route::*;


fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;

    let mut route = SantaRoute::new();
    route.visit_house(DeliveryBy::Santa);

    for c in input.chars() {
        let new_direction: Vector2 = match c {
            '>' => Vector2::right(),
            '<' => Vector2::left(),
            '^' => Vector2::up(),
            'v' => Vector2::down(),
            _ => Vector2::new(0, 0),
        };
        route.santa_position.add(new_direction);
        route.visit_house(DeliveryBy::Santa);
    }

    println!("Santa visited {} houses at elast once", route.visited_houses.len());

    let mut route = SantaRoute::new();
    route.visit_house(DeliveryBy::Santa);
    route.visit_house(DeliveryBy::RoboSanta);
    let mut is_santa_turn = true;
    for c in input.chars() {
        let new_direction: Vector2 = match c {
            '>' => Vector2::right(),
            '<' => Vector2::left(),
            '^' => Vector2::up(),
            'v' => Vector2::down(),
            _ => Vector2::new(0, 0),
        };
        if is_santa_turn {
            route.santa_position.add(new_direction);
            route.visit_house(DeliveryBy::Santa);
        } else {
            route.robo_santa_position.add(new_direction);
            route.visit_house(DeliveryBy::RoboSanta);
        }
        is_santa_turn = !is_santa_turn;
    }
    
    println!("Santa with robostanta visited {} houses at elast once", route.visited_houses.len());

    Ok(())
}
