use utils::vector2::*;

pub enum Vector2CreationError {}

pub trait Vector2Ext {
    fn new_from_str(s: &str) -> Result<Self, Vector2CreationError>
    where
        Self: Sized;
}

impl Vector2Ext for Vector2 {
    fn new_from_str(s: &str) -> Result<Self, Vector2CreationError> {
        // extract coords from string s
        Ok(Vector2::new(0, 0))
    }
}

pub enum CommandCreationError {
    InvalidOperation,
    InvalidStart,
    InvalidEnd,
}

pub enum CommandOperation {
    TurnOn,
    TurnOff,
    Toggle,
}

pub struct Command {
    operation: CommandOperation,
    start: Vector2,
    end: Vector2,
}

impl Command {
    pub fn new(s: &str) -> Result<Self, CommandCreationError> {
        Ok(Self {
            operation: CommandOperation::TurnOn,
            start: Vector2::new(0, 0),
            end: Vector2::new(1, 1),
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
                Command::new("toggle on 0,0 through 999,999"),
                Err(CommandCreationError::InvalidOperation)
            ),
            "Should return error on invalid operation name"
        );

        assert!(matches!(
            Command::new("turn on a,0 through 999,999"),
            Err(CommandCreationError::InvalidStart)
        ));
        assert!(matches!(
            Command::new("turn on 0 through 999,999"),
            Err(CommandCreationError::InvalidStart)
        ));
        assert!(matches!(
            Command::new("turn on 0,0,0 through 999,999"),
            Err(CommandCreationError::InvalidStart)
        ));

        assert!(matches!(
            Command::new("turn on 0,0 through 999,a"),
            Err(CommandCreationError::InvalidEnd)
        ));
        assert!(matches!(
            Command::new("turn on 0,0 through 999"),
            Err(CommandCreationError::InvalidEnd)
        ));
        assert!(matches!(
            Command::new("turn on 0,0 through 999,0,0"),
            Err(CommandCreationError::InvalidEnd)
        ));
        assert!(matches!(
            Command::new("turn on 0,0 through 999,0.2"),
            Err(CommandCreationError::InvalidEnd)
        ));

        assert!(matches!(
            Command::new("turn on 0,0 through 999,999"),
            Ok(Command {
                operation: CommandOperation::TurnOn,
                start: Vector2 { x: 0, y: 0 },
                end: Vector2 { x: 999, y: 999 }
            })
        ));
        assert!(matches!(
            Command::new("toggle 0,0 through 999,0"),
            Ok(Command {
                operation: CommandOperation::Toggle,
                start: Vector2 { x: 0, y: 0 },
                end: Vector2 { x: 999, y: 0 }
            })
        ));
        assert!(matches!(
            Command::new("turn off 499,499 through 500,500"),
            Ok(Command {
                operation: CommandOperation::TurnOff,
                start: Vector2 { x: 499, y: 499 },
                end: Vector2 { x: 500, y: 500 }
            })
        ));
    }
}
