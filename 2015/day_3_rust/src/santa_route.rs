use std::collections::HashMap;
use crate::Vector2;

pub enum DeliveryBy {
    Santa,
    RoboSanta
}

pub struct SantaRoute {
    pub santa_position: Vector2,
    pub robo_santa_position: Vector2,
    pub visited_houses: HashMap<String, i32>,
}

impl SantaRoute {
    pub fn new() -> Self {
        Self {
            santa_position: Vector2::new(0, 0),
            robo_santa_position: Vector2::new(0, 0),
            visited_houses: HashMap::new(),
        }
    }

    pub fn visit_house(&mut self, delivered_by: DeliveryBy) {
        let coords = match delivered_by {
            DeliveryBy::Santa => self.santa_position.get_coords_str(),
            DeliveryBy::RoboSanta => self.robo_santa_position.get_coords_str(),
        };
        let count = self.visited_houses.entry(coords).or_insert(0);
        *count += 1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_creation() {
        let route = SantaRoute::new();

        assert!(matches!(route.santa_position, Vector2 { x: 0, y: 0 }));
    }

    #[test]
    fn test_visit_house() {
        let mut route = SantaRoute::new();
        route.visit_house(DeliveryBy::Santa);

        route.santa_position.add(Vector2::right());
        route.visit_house(DeliveryBy::Santa);

        route.santa_position.add(Vector2::left());
        route.visit_house(DeliveryBy::Santa);

        assert_eq!(route.visited_houses.len(), 2);

        assert_eq!(route.visited_houses.get( &Vector2::new(1,0).get_coords_str() ).unwrap(), &1);
        assert_eq!(route.visited_houses.get( &Vector2::new(0,0).get_coords_str() ).unwrap(), &2);
    }

    #[test]
    fn test_robo_santa() {
        let mut route = SantaRoute::new();
        route.visit_house(DeliveryBy::RoboSanta);

        route.santa_position.add(Vector2::right());
        route.visit_house(DeliveryBy::RoboSanta);

        route.santa_position.add(Vector2::left());
        route.visit_house(DeliveryBy::RoboSanta);

        assert_eq!(route.visited_houses.len(), 1);
    }
}