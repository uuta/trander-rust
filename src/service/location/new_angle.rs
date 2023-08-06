use crate::service::location::{DetailedDirectionType, DirectionType};
use mockall::automock;
use rand::Rng;

#[automock]
pub trait NewAngle {
    fn new_angle(&self, direction_type: DirectionType) -> f64;
    fn detailed_direction(&self, angle: f64) -> &str;
}

pub enum DirectionCount {
    Single(f64, f64),
    Double((f64, f64), (f64, f64)),
}

pub struct NewAngleService;

impl NewAngle for NewAngleService {
    fn new_angle(&self, direction_type: DirectionType) -> f64 {
        let direction_count = direction_type.to_angle(direction_type);
        direction_count.gen_angle()
    }

    fn detailed_direction(&self, angle: f64) -> &str {
        DetailedDirectionType::from_angle(angle).to_str()
    }
}

impl DirectionType {
    pub fn to_angle(&self, direction_type: DirectionType) -> DirectionCount {
        match direction_type {
            DirectionType::North => DirectionCount::Double((315.0, 360.0), (0.0, 45.0)),
            DirectionType::East => DirectionCount::Single(45.0, 135.0),
            DirectionType::South => DirectionCount::Single(135.0, 225.0),
            DirectionType::West => DirectionCount::Single(225.0, 315.0),
        }
    }
}

impl DetailedDirectionType {
    pub fn from_angle(angle: f64) -> DetailedDirectionType {
        match angle {
            0.0..=22.5 => DetailedDirectionType::North,
            22.5..=67.5 => DetailedDirectionType::NorthEast,
            67.5..=112.5 => DetailedDirectionType::East,
            112.5..=157.5 => DetailedDirectionType::SouthEast,
            157.5..=202.5 => DetailedDirectionType::South,
            202.5..=247.5 => DetailedDirectionType::SouthWest,
            247.5..=292.5 => DetailedDirectionType::West,
            292.5..=337.5 => DetailedDirectionType::NorthWest,
            337.5..=360.0 => DetailedDirectionType::North,
            _ => panic!("Invalid angle: {}", angle),
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            DetailedDirectionType::North => "North",
            DetailedDirectionType::NorthEast => "NorthEast",
            DetailedDirectionType::East => "East",
            DetailedDirectionType::SouthEast => "SouthEast",
            DetailedDirectionType::South => "South",
            DetailedDirectionType::SouthWest => "SouthWest",
            DetailedDirectionType::West => "West",
            DetailedDirectionType::NorthWest => "NorthWest",
        }
    }
}

impl DirectionCount {
    pub fn gen_angle(&self) -> f64 {
        let mut rng = rand::thread_rng();
        match self {
            DirectionCount::Single(min, max) => rng.gen_range(*min..*max),
            DirectionCount::Double(range1, range2) => {
                if rng.gen_bool(0.5) {
                    rng.gen_range(range1.0..range1.1)
                } else {
                    rng.gen_range(range2.0..range2.1)
                }
            }
        }
    }
    pub fn detailed_angle(&self, angle: f64) -> &str {
        DetailedDirectionType::to_angle(&self, angle).to_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_angle() {
        let direction_types = [
            DirectionType::North,
            DirectionType::East,
            DirectionType::South,
            DirectionType::West,
        ];

        for direction_type in direction_types.iter() {
            let service = NewAngleService;
            let new_angle = service.new_angle(*direction_type);

            match direction_type {
                DirectionType::North => {
                    assert!(
                        new_angle >= 315.0 && new_angle <= 360.0
                            || new_angle >= 0.0 && new_angle <= 45.0
                    );
                }
                DirectionType::East => {
                    assert!(new_angle >= 45.0 && new_angle <= 135.0);
                }
                DirectionType::South => {
                    assert!(new_angle >= 135.0 && new_angle <= 225.0);
                }
                DirectionType::West => {
                    assert!(new_angle >= 225.0 && new_angle <= 315.0);
                }
            }
        }
    }

    #[test]
    fn test_gen_angle() {
        let direction_types = [
            DirectionType::North,
            DirectionType::East,
            DirectionType::South,
            DirectionType::West,
        ];

        for direction_type in direction_types.iter() {
            let direction_count = direction_type.to_angle(*direction_type);
            let angle = direction_count.gen_angle();

            match direction_count {
                DirectionCount::Single(min, max) => {
                    assert!(angle >= min && angle <= max);
                }
                DirectionCount::Double(range1, range2) => {
                    assert!(
                        (angle >= range1.0 && angle <= range1.1)
                            || (angle >= range2.0 && angle <= range2.1)
                    );
                }
            }
        }
    }
}
