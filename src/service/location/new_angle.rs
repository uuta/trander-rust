use crate::service::location::{DetailedDirectionType, DirectionType};
use mockall::automock;
use rand::Rng;

#[automock]
pub trait NewAngle: Send {
    fn new_angle(&self, direction_type: DirectionType) -> f64;
    fn detailed_direction(&self, angle: f64) -> String;
}

pub enum DirectionCount {
    Single(f64, f64),
    Double((f64, f64), (f64, f64)),
}

pub struct NewAngleService;

impl NewAngle for NewAngleService {
    fn new_angle(&self, direction_type: DirectionType) -> f64 {
        let direction_count = direction_type.from_angle(direction_type);
        direction_count.gen_angle()
    }

    fn detailed_direction(&self, angle: f64) -> String {
        DetailedDirectionType::from_angle(angle)
            .to_str()
            .to_string()
    }
}

impl DirectionType {
    pub fn from_angle(&self, direction_type: DirectionType) -> DirectionCount {
        match direction_type {
            DirectionType::All => DirectionCount::Single(0.0, 360.0),
            DirectionType::North => DirectionCount::Double((315.0, 360.0), (0.0, 45.0)),
            DirectionType::East => DirectionCount::Single(45.0, 135.0),
            DirectionType::South => DirectionCount::Single(135.0, 225.0),
            DirectionType::West => DirectionCount::Single(225.0, 315.0),
        }
    }
}

impl DetailedDirectionType {
    pub fn from_angle(angle: f64) -> DetailedDirectionType {
        if angle >= 0.0 && angle <= 22.5 {
            DetailedDirectionType::North
        } else if angle <= 67.5 {
            DetailedDirectionType::NorthEast
        } else if angle <= 112.5 {
            DetailedDirectionType::East
        } else if angle <= 157.5 {
            DetailedDirectionType::SouthEast
        } else if angle <= 202.5 {
            DetailedDirectionType::South
        } else if angle <= 247.5 {
            DetailedDirectionType::SouthWest
        } else if angle <= 292.5 {
            DetailedDirectionType::West
        } else if angle <= 337.5 {
            DetailedDirectionType::NorthWest
        } else if angle <= 360.0 {
            DetailedDirectionType::North
        } else {
            panic!("Invalid angle: {}", angle)
        }
    }
    pub fn to_str(&self) -> &'static str {
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
                DirectionType::All => {
                    assert!(new_angle >= 0.0 && new_angle <= 360.0);
                }
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
            let direction_count = direction_type.from_angle(*direction_type);
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
