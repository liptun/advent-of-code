use utils::vector2::*;

pub enum CommandCreationError {}

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
