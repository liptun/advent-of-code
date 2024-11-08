use utils::vector2::Vector2;

use crate::command::*;

pub struct Lights {
    pub grid: Vec<u32>,
    pub size: usize,
}

impl Lights {
    pub fn new() -> Self {
        let size = 1000_usize;
        Self {
            size,
            grid: vec![0; size * size],
        }
    }

    fn is_pos_out_of_bounds(&self, pos: &Vector2) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.size as i32 || pos.y >= self.size as i32
    }

    pub fn vector_to_index(&self, pos: &Vector2) -> usize {
        (pos.y * self.size as i32 + pos.x) as usize
    }

    pub fn index_to_vector(&self, index: &usize) -> Vector2 {
        let x = index % self.size;
        let y = index / self.size;
        Vector2::new(x as i32, y as i32)
    }

    fn get(&self, pos: &Vector2) -> Option<&u32> {
        if self.is_pos_out_of_bounds(&pos) {
            return None;
        }

        let index = self.vector_to_index(&pos);

        if let Some(light) = self.grid.get(index) {
            Some(light)
        } else {
            None
        }
    }

    fn set(&mut self, pos: &Vector2, state: u32) -> bool {
        if self.is_pos_out_of_bounds(&pos) {
            return false;
        }

        let index = self.vector_to_index(&pos);

        if let Some(light) = self.grid.get_mut(index) {
            *light = state;
            return true;
        }

        false
    }

    pub fn exec(&mut self, command: &Command) -> () {
        let mut x_asix_range: Vec<usize> = vec![command.start.x as usize, command.end.x as usize];
        let mut y_asix_range: Vec<usize> = vec![command.start.y as usize, command.end.y as usize];
        x_asix_range.sort();
        y_asix_range.sort();

        let x_start = *x_asix_range.get(0).unwrap_or(&0);
        let x_end = *x_asix_range.get(1).unwrap_or(&0);
        let y_start = *y_asix_range.get(0).unwrap_or(&0);
        let y_end = *y_asix_range.get(1).unwrap_or(&0);

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                let pos = Vector2::new(x as i32, y as i32);

                if let Some(light) = self.get(&pos) {
                    match command.operation {
                        CommandOperation::TurnOn => self.set(&pos, 1),
                        CommandOperation::TurnOff => self.set(&pos, 0),
                        CommandOperation::Toggle => self.set(&pos, if *light == 0 { 1 } else { 0 }),
                    };
                }
            }
        }
    }

    pub fn exec_pt2(&mut self, command: &Command) -> () {
        let mut x_asix_range: Vec<usize> = vec![command.start.x as usize, command.end.x as usize];
        let mut y_asix_range: Vec<usize> = vec![command.start.y as usize, command.end.y as usize];
        x_asix_range.sort();
        y_asix_range.sort();

        let x_start = *x_asix_range.get(0).unwrap_or(&0);
        let x_end = *x_asix_range.get(1).unwrap_or(&0);
        let y_start = *y_asix_range.get(0).unwrap_or(&0);
        let y_end = *y_asix_range.get(1).unwrap_or(&0);

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                let pos = Vector2::new(x as i32, y as i32);

                if let Some(light) = self.get(&pos) {
                    match command.operation {
                        CommandOperation::TurnOn => self.set(&pos, *light + 1),
                        CommandOperation::TurnOff => self.set(&pos, light.saturating_sub(1)),
                        CommandOperation::Toggle => self.set(&pos, *light + 2),
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lights_creation() {
        let lights = Lights::new();

        assert_eq!(lights.grid.len(), lights.size * lights.size);

        assert!(
            matches!(lights.get(&Vector2::new(0, 0)), Some(0)),
            "should get light at 0,0"
        );
        assert!(
            matches!(lights.get(&Vector2::new(999, 999)), Some(0)),
            "should get light at 999,999"
        );
        assert!(
            matches!(lights.get(&Vector2::new(1000, 1000)), None),
            "should get None at 1000,1000, out of bounds"
        );
        assert!(
            matches!(lights.get(&Vector2::new(-1, 0)), None),
            "should get None at -1,0, out of bounds"
        );
        assert!(
            matches!(lights.get(&Vector2::new(0, -1)), None),
            "should get None at 0,-1 out of bounds"
        );
    }

    #[test]
    fn test_lights_set() {
        let mut lights = Lights::new();

        assert!(matches!(lights.get(&Vector2::new(0, 0)), Some(0)));

        assert_eq!(lights.set(&Vector2::new(0, 0), 1), true);
        assert!(matches!(lights.get(&Vector2::new(0, 0)), Some(1)));

        assert_eq!(lights.set(&Vector2::new(999, 999), 1), true);
        assert!(matches!(lights.get(&Vector2::new(999, 999)), Some(1)));

        assert_eq!(lights.set(&Vector2::new(1000, 1000), 1), false);
    }

    #[test]
    fn test_lights_exec() {
        let mut lights = Lights::new();

        lights.exec(&Command::new("turn on 0,0 through 999,999").unwrap());
        assert!(matches!(lights.get(&Vector2::new(500, 500)), Some(1)));

        lights.exec(&Command::new("toggle 0,0 through 999,0").unwrap());
        assert!(matches!(lights.get(&Vector2::new(500, 0)), Some(0)));

        assert!(matches!(lights.get(&Vector2::new(500, 500)), Some(1)));
        lights.exec(&Command::new("turn off 499,499 through 500,500").unwrap());
        assert!(matches!(lights.get(&Vector2::new(500, 500)), Some(0)));
    }
}
