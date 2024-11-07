#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&mut self, vector_to_add: Vector2) -> () {
        self.x += vector_to_add.x;
        self.y += vector_to_add.y;
    }

    pub fn up() -> Self {
        Vector2::new(0, -1)
    }

    pub fn down() -> Self {
        Vector2::new(0, 1)
    }

    pub fn left() -> Self {
        Vector2::new(-1, 0)
    }

    pub fn right() -> Self {
        Vector2::new(1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vector = Vector2::new(2, 4);
        assert_eq!(vector.x, 2);
        assert_eq!(vector.y, 4);
    }


    #[test]
    fn test_vector_add() {
        let mut vector = Vector2::new(3, 5);
        assert!(matches!(vector, Vector2 { x: 3, y: 5 }));
        vector.add(Vector2::new(0, 1));
        assert!(matches!(vector, Vector2 { x: 3, y: 6 }));
        vector.add(Vector2::new(1, 0));
        assert!(matches!(vector, Vector2 { x: 4, y: 6 }));
        vector.add(Vector2::new(2, -3));
        assert!(matches!(vector, Vector2 { x: 6, y: 3 }));
        vector.add(Vector2::new(0, -20));
        assert!(matches!(vector, Vector2 { x: 6, y: -17 }));
    }

    #[test]
    fn test_direction_helpers() {
        assert!(matches!(Vector2::up(), Vector2 { x: 0, y: -1 }));
        assert!(matches!(Vector2::down(), Vector2 { x: 0, y: 1 }));
        assert!(matches!(Vector2::left(), Vector2 { x: -1, y: 0 }));
        assert!(matches!(Vector2::right(), Vector2 { x: 1, y: 0 }));

        let mut vector = Vector2::new(0, 0);
        assert!(matches!(vector, Vector2 { x: 0, y: 0 }));
        vector.add(Vector2::right());
        assert!(matches!(vector, Vector2 { x: 1, y: 0 }));
    }
}
