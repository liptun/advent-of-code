use utils::vector2::*;
use crate::vector2_ext::*;

#[derive(Debug)]
pub enum CommandCreationError {
    InvalidOperation,
    MissingSeparator,
    InvalidCoords,
}

#[derive(Debug)]
pub enum CommandOperation {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
pub struct Command {
    pub operation: CommandOperation,
    pub start: Vector2,
    pub end: Vector2,
}

impl Command {
    pub fn new(s: &str) -> Result<Self, CommandCreationError> {
        let trim_from;
        let operation: CommandOperation;

        if s.starts_with("turn on") {
            trim_from = 8;
            operation = CommandOperation::TurnOn
        } else if s.starts_with("turn off") {
            trim_from = 9;
            operation = CommandOperation::TurnOff
        } else if s.starts_with("toggle") {
            trim_from = 7;
            operation = CommandOperation::Toggle
        } else {
            return Err(CommandCreationError::InvalidOperation);
        };

        let coords_pair = &s[trim_from..];

        if !coords_pair.contains("through") {
            return Err(CommandCreationError::MissingSeparator);
        }

        let coords: Vec<&str> = coords_pair.split("through").map(|c| c.trim()).collect();

        if coords.len() != 2 {
            return Err(CommandCreationError::InvalidCoords);
        }

        let start = if let Ok(start) = Vector2::new_from_str(coords[0]) {
            start
        } else {
            return Err(CommandCreationError::InvalidCoords);
        };
        let end = if let Ok(end) = Vector2::new_from_str(coords[1]) {
            end
        } else {
            return Err(CommandCreationError::InvalidCoords);
        };

        Ok(Self {
            operation,
            start,
            end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert!(
            matches!(
                Command::new("keep on 0,0 through 999,999"),
                Err(CommandCreationError::InvalidOperation)
            ),
            "Should return error on invalid operation name"
        );
        assert!(
            matches!(
                Command::new("turn on 0,0 999,999"),
                Err(CommandCreationError::MissingSeparator)
            ),
            "Should return error on missing separator"
        );
        assert!(
            matches!(
                Command::new("turn off 0,0 through"),
                Err(CommandCreationError::InvalidCoords)
            ),
            "Should return error on invalid coords"
        );

        assert!(
            matches!(
                Command::new("turn on 0,0 through 999,999"),
                Ok(Command {
                    operation: CommandOperation::TurnOn,
                    start: Vector2 { x: 0, y: 0 },
                    end: Vector2 { x: 999, y: 999 }
                }),
            ),
            "Should create command 1"
        );
        assert!(
            matches!(
                Command::new("toggle 0,0 through 999,0"),
                Ok(Command {
                    operation: CommandOperation::Toggle,
                    start: Vector2 { x: 0, y: 0 },
                    end: Vector2 { x: 999, y: 0 }
                })
            ),
            "Should create command 2"
        );
        assert!(
            matches!(
                Command::new("turn off 499,499 through 500,500"),
                Ok(Command {
                    operation: CommandOperation::TurnOff,
                    start: Vector2 { x: 499, y: 499 },
                    end: Vector2 { x: 500, y: 500 }
                })
            ),
            "Should create command 3"
        );
    }
}
