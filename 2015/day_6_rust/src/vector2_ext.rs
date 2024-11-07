use utils::vector2::*;

pub enum Vector2CreationError {
    EmptyValue,
    InvalidFormat,
}

pub trait Vector2Ext {
    fn new_from_str(s: &str) -> Result<Self, Vector2CreationError>
    where
        Self: Sized;
}

impl Vector2Ext for Vector2 {
    fn new_from_str(s: &str) -> Result<Self, Vector2CreationError> {
        if s == "" {
            return Err(Vector2CreationError::EmptyValue);
        }

        let coords: Vec<i32> = s.split(",").filter_map(|v| v.parse().ok()).collect();

        if coords.len() != 2 {
            return Err(Vector2CreationError::InvalidFormat);
        }

        Ok(Vector2::new(coords[0], coords[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_from_str() {
        assert!(
            matches!(
                Vector2::new_from_str(""),
                Err(Vector2CreationError::EmptyValue)
            ),
            "Should return creation error"
        );
        assert!(
            matches!(
                Vector2::new_from_str("33"),
                Err(Vector2CreationError::InvalidFormat)
            ),
            "Should return creation error"
        );
        assert!(
            matches!(
                Vector2::new_from_str("3,3,3"),
                Err(Vector2CreationError::InvalidFormat)
            ),
            "Should return creation error"
        );
        assert!(
            matches!(
                Vector2::new_from_str("a,3"),
                Err(Vector2CreationError::InvalidFormat)
            ),
            "Should return creation error"
        );
        assert!(
            matches!(
                Vector2::new_from_str("3,a"),
                Err(Vector2CreationError::InvalidFormat)
            ),
            "Should return creation error"
        );

        assert!(
            matches!(Vector2::new_from_str("0,0"), Ok(Vector2 { x: 0, y: 0 })),
            "Should create Vector2"
        );
        assert!(
            matches!(
                Vector2::new_from_str("999,999"),
                Ok(Vector2 { x: 999, y: 999 })
            ),
            "Should create Vector2"
        );
    }
}
