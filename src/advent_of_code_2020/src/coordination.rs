use std::f64::consts::PI;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UPoint {
    pub x: usize,
    pub y: usize,
}

impl UPoint {
    pub fn x_capped_transform(&self, x: usize, y: usize, cap: usize) -> UPoint {
        UPoint {
            x: (self.x + x) % (cap + 1),
            y: self.y + y,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct IPoint {
    pub x: i32,
    pub y: i32,
}

impl IPoint {
    pub fn from(x: i32, y: i32) -> IPoint {
        IPoint { x, y }
    }

    pub fn move_compass(&self, direction: CompassDirection, range: i32) -> IPoint {
        match direction {
            CompassDirection::North => IPoint {
                x: self.x,
                y: self.y + range,
            },
            CompassDirection::East => IPoint {
                x: self.x + range,
                y: self.y,
            },
            CompassDirection::South => IPoint {
                x: self.x,
                y: self.y - range,
            },
            CompassDirection::West => IPoint {
                x: self.x - range,
                y: self.y,
            },
        }
    }

    pub fn rotate_right(&self, degree: StandardRotation) -> IPoint {
        match degree {
            StandardRotation::Degree90 => self.rotate_left(StandardRotation::Degree270),
            StandardRotation::Degree180 => self.rotate_left(StandardRotation::Degree180),
            StandardRotation::Degree270 => self.rotate_left(StandardRotation::Degree90),
        }
    }

    pub fn rotate_left(&self, degree: StandardRotation) -> IPoint {
        let sin = degree.sin();
        let cos = degree.cos();
        let x = cos * self.x - sin * self.y;
        let y = sin * self.x + cos * self.y;
        IPoint { x, y }
    }

    pub fn get_manhatten_distance(&self, to: IPoint) -> i32 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}

impl std::ops::Mul<i32> for IPoint {
    type Output = IPoint;

    fn mul(self, rhs: i32) -> IPoint {
        IPoint {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add<IPoint> for IPoint {
    type Output = IPoint;

    fn add(self, rhs: IPoint) -> IPoint {
        IPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct FPoint {
    pub x: f64,
    pub y: f64,
}

impl FPoint {
    pub fn from(x: f64, y: f64) -> FPoint {
        FPoint { x, y }
    }

    pub fn move_compass(&self, direction: CompassDirection, range: u32) -> FPoint {
        match direction {
            CompassDirection::North => FPoint {
                x: self.x,
                y: self.y + f64::from(range),
            },
            CompassDirection::East => FPoint {
                x: self.x + f64::from(range),
                y: self.y,
            },
            CompassDirection::South => FPoint {
                x: self.x,
                y: self.y - f64::from(range),
            },
            CompassDirection::West => FPoint {
                x: self.x - f64::from(range),
                y: self.y,
            },
        }
    }

    pub fn rotate_right(&self, degree: f64) -> FPoint {
        let left_degree = 360.0 - degree;
        self.rotate_left(left_degree)
    }

    pub fn rotate_left(&self, degree: f64) -> FPoint {
        let radian = (degree / 180.0) * PI;
        let sin = radian.sin();
        let cos = radian.cos();
        let x = cos * self.x - sin * self.y;
        let y = sin * self.x + cos * self.y;
        FPoint { x, y }
    }

    pub fn get_manhatten_distance(&self, to: FPoint) -> f64 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum CompassDirection {
    North,
    South,
    East,
    West,
}

impl CompassDirection {
    pub fn rotate_right(&self, degree: StandardRotation) -> CompassDirection {
        match (self, degree) {
            (CompassDirection::North, StandardRotation::Degree90) => CompassDirection::East,
            (CompassDirection::North, StandardRotation::Degree180) => CompassDirection::South,
            (CompassDirection::North, StandardRotation::Degree270) => CompassDirection::West,
            (CompassDirection::East, StandardRotation::Degree90) => CompassDirection::South,
            (CompassDirection::East, StandardRotation::Degree180) => CompassDirection::West,
            (CompassDirection::East, StandardRotation::Degree270) => CompassDirection::North,
            (CompassDirection::South, StandardRotation::Degree90) => CompassDirection::West,
            (CompassDirection::South, StandardRotation::Degree180) => CompassDirection::North,
            (CompassDirection::South, StandardRotation::Degree270) => CompassDirection::East,
            (CompassDirection::West, StandardRotation::Degree90) => CompassDirection::North,
            (CompassDirection::West, StandardRotation::Degree180) => CompassDirection::East,
            (CompassDirection::West, StandardRotation::Degree270) => CompassDirection::South,
        }
    }

    pub fn rotate_left(&self, degree: StandardRotation) -> CompassDirection {
        match (self, degree) {
            (CompassDirection::North, StandardRotation::Degree90) => CompassDirection::West,
            (CompassDirection::North, StandardRotation::Degree180) => CompassDirection::South,
            (CompassDirection::North, StandardRotation::Degree270) => CompassDirection::East,
            (CompassDirection::East, StandardRotation::Degree90) => CompassDirection::North,
            (CompassDirection::East, StandardRotation::Degree180) => CompassDirection::West,
            (CompassDirection::East, StandardRotation::Degree270) => CompassDirection::South,
            (CompassDirection::South, StandardRotation::Degree90) => CompassDirection::East,
            (CompassDirection::South, StandardRotation::Degree180) => CompassDirection::North,
            (CompassDirection::South, StandardRotation::Degree270) => CompassDirection::West,
            (CompassDirection::West, StandardRotation::Degree90) => CompassDirection::South,
            (CompassDirection::West, StandardRotation::Degree180) => CompassDirection::East,
            (CompassDirection::West, StandardRotation::Degree270) => CompassDirection::North,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StandardRotation {
    Degree90,
    Degree180,
    Degree270,
}

impl StandardRotation {
    fn sin(&self) -> i32 {
        match self {
            StandardRotation::Degree90 => 1,
            StandardRotation::Degree180 => 0,
            StandardRotation::Degree270 => -1,
        }
    }

    fn cos(&self) -> i32 {
        match self {
            StandardRotation::Degree90 => 0,
            StandardRotation::Degree180 => -1,
            StandardRotation::Degree270 => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotation_matrix_test() {
        let p = FPoint::from(1.0, 2.0);
        let p_90 = p.rotate_right(90.0);
        assert!(2.0 - p_90.x < 1e-10);
        assert!(-1.0 - p_90.y < 1e-10);

        let p_180 = p.rotate_right(180.0);
        assert!(-1.0 - p_180.x < 1e-10);
        assert!(-2.0 - p_180.y < 1e-10);

        let p_270 = p.rotate_right(270.0);
        assert!(-2.0 - p_270.x < 1e-10);
        assert!(1.0 - p_270.y < 1e-10);
    }

    #[test]
    fn standard_rotation_test() {
        let p = IPoint::from(1, 2);
        let p_90 = p.rotate_right(StandardRotation::Degree90);
        assert_eq!(2, p_90.x);
        assert_eq!(-1, p_90.y);

        let p_180 = p.rotate_right(StandardRotation::Degree180);
        assert_eq!(-1, p_180.x);
        assert_eq!(-2, p_180.y);

        let p_270 = p.rotate_right(StandardRotation::Degree270);
        assert_eq!(-2, p_270.x);
        assert_eq!(1, p_270.y);
    }
}
