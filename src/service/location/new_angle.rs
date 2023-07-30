use crate::service::location::DirectionType;
use mockall::automock;
use rand::Rng;

#[automock]
pub trait NewAngle {
    fn new_angle(&self, direction_type: DirectionType) -> f64;
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
