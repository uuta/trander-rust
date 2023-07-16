use crate::service::location::new_angle::{DirectionType, NewAngle};
use crate::service::location::new_dest::NewDest;
use mockall::automock;

pub mod new_angle;
pub mod new_dest;
pub mod new_distance;

#[automock]
pub trait LocationService {
    fn location(&self) -> (f64, f64);
}

/// lat: latitute
/// lng: longitude
/// distance: distance from the point (1km = 1000.0)
/// direction_type: DirectionType (see new_angle.rs)
pub struct ImplLocationService {
    lng: f64,
    lat: f64,
    distance: f64,
    direction_type: DirectionType,
    new_angle_service: Box<dyn NewAngle>,
    new_dest_service: Box<dyn NewDest>,
}

impl ImplLocationService {
    pub fn new(
        lng: f64,
        lat: f64,
        distance: f64,
        direction_type: DirectionType,
        new_angle_service: Box<dyn NewAngle>,
        new_dest_service: Box<dyn NewDest>,
    ) -> Self {
        Self {
            lng,
            lat,
            distance,
            direction_type,
            new_angle_service,
            new_dest_service,
        }
    }
}

impl LocationService for ImplLocationService {
    fn location(&self) -> (f64, f64) {
        let angle = self.new_angle_service.new_angle(self.direction_type);
        self.new_dest_service
            .new_dest(self.lng, self.lat, angle, self.distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::location::new_angle::MockNewAngle;
    use crate::service::location::new_dest::MockNewDest;
    use mockall::predicate::*;

    #[test]
    fn test_location() {
        let mut mock_angle_service = MockNewAngle::new();
        mock_angle_service
            .expect_new_angle()
            .with(eq(DirectionType::East))
            .return_const(45.0); // Always return 45.0

        let mut mock_dest_service = MockNewDest::new();
        mock_dest_service
            .expect_new_dest()
            .with(eq(139.767125), eq(35.681236), eq(45.0), eq(100000.0))
            .return_const((35.6761685462078, 140.87174397802116)); // Always return these coordinates

        let location_service = ImplLocationService::new(
            139.767125,
            35.681236,
            100000.0,
            DirectionType::East,
            Box::new(mock_angle_service),
            Box::new(mock_dest_service),
        );
        let (lat, lng) = location_service.location();
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
    }
}
