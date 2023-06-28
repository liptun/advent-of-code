use std::cmp;

#[derive(Debug)]
pub enum PresentBoxCreationError {
    InvalidDimensions,
}

pub struct PresentBox {
    length: u32,
    width: u32,
    height: u32,
}

impl PresentBox {
    pub fn new(s: &str) -> Result<Self, PresentBoxCreationError> {
        let dimensions: Vec<u32> = s
            .split("x")
            .map(|d| d.parse::<u32>().unwrap_or(0))
            .collect();

        if dimensions.contains(&0) || dimensions.len() != 3 {
            return Err(PresentBoxCreationError::InvalidDimensions);
        }

        let length = dimensions[0];
        let width = dimensions[1];
        let height = dimensions[2];

        Ok(Self {
            length,
            width,
            height,
        })
    }

    fn get_surface_a(&self) -> u32 {
        self.length * self.width
    }

    fn get_surface_b(&self) -> u32 {
        self.width * self.height
    }

    fn get_surface_c(&self) -> u32 {
        self.height * self.length
    }

    fn get_smallest_face(&self) -> u32 {
        cmp::min(
            self.get_surface_a(),
            cmp::min(self.get_surface_b(), self.get_surface_c()),
        )
    }

    fn get_surface_area(&self) -> u32 {
        self.get_surface_a() * 2 + self.get_surface_b() * 2 + self.get_surface_c() * 2
    }

    pub fn get_surface_area_extra(&self) -> u32 {
        self.get_surface_area() + self.get_smallest_face()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_creation() {
        assert!(matches!(
            PresentBox::new("0x3x5"),
            Err(PresentBoxCreationError::InvalidDimensions)
        ));

        assert!(matches!(
            PresentBox::new("2x3"),
            Err(PresentBoxCreationError::InvalidDimensions)
        ));

        assert!(matches!(
            PresentBox::new("2x-3x4"),
            Err(PresentBoxCreationError::InvalidDimensions)
        ));

        assert!(matches!(
            PresentBox::new("2x3x3.14"),
            Err(PresentBoxCreationError::InvalidDimensions)
        ));

        assert!(matches!(
            PresentBox::new("2x3x4x5"),
            Err(PresentBoxCreationError::InvalidDimensions)
        ));

        assert!(matches!(
            PresentBox::new("ax3x4"),
            Err(PresentBoxCreationError::InvalidDimensions)
        ));

        assert!(matches!(
            PresentBox::new("2x3x4"),
            Ok(PresentBox {
                length: 2,
                width: 3,
                height: 4
            })
        ));

        assert!(matches!(
            PresentBox::new("21x37x69"),
            Ok(PresentBox {
                length: 21,
                width: 37,
                height: 69
            })
        ));
    }

    #[test]
    fn test_box_surface_area() {
        assert_eq!(
            PresentBox::new("2x3x4")
                .expect("Present box should create")
                .get_surface_area(),
            52
        );
    }

    #[test]
    fn test_box_smallest_face_surface() {
        assert_eq!(
            PresentBox::new("2x3x4")
                .expect("Present box should create")
                .get_smallest_face(),
            6
        );
        assert_eq!(
            PresentBox::new("3x4x2")
                .expect("Present box should create")
                .get_smallest_face(),
            6
        );
        assert_eq!(
            PresentBox::new("4x2x3")
                .expect("Present box should create")
                .get_smallest_face(),
            6
        );
        assert_eq!(
            PresentBox::new("4x3x2")
                .expect("Present box should create")
                .get_smallest_face(),
            6
        );
    }

    #[test]
    fn test_box_surface_area_extra() {
        assert_eq!(
            PresentBox::new("2x3x4")
                .expect("Present box should create")
                .get_surface_area_extra(),
            58
        );
        assert_eq!(
            PresentBox::new("1x1x10")
                .expect("Present box should create")
                .get_surface_area_extra(),
            43
        );
    }
}
