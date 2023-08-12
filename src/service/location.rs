use crate::service::location::new_angle::NewAngle;
use crate::service::location::new_dest::NewDest;
use crate::service::location::new_distance::NewDistance;
use mockall::automock;

pub mod new_angle;
pub mod new_dest;
pub mod new_distance;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DirectionType {
    All,
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DetailedDirectionType {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[automock]
pub trait LocationService {
    fn gen(&mut self) -> ();
    fn get(&mut self) -> (f64, f64);
    fn format(&mut self) -> String;
    fn concat(&mut self) -> String;
    fn distance(&mut self, target_lng: f64, target_lat: f64) -> f64;
    fn detailed_direction(&mut self) -> String;
}

/// lat: latitute
/// lng: longitude
/// distance: distance from the point (1km = 1000.0)
/// direction_type: DirectionType
pub struct ImplLocationService {
    lng: f64,
    lat: f64,
    distance: f64,
    direction_type: DirectionType,
    angle: f64,
    new_angle_service: Box<dyn NewAngle + Send>,
    new_dest_service: Box<dyn NewDest + Send>,
    new_distance_service: Box<dyn NewDistance + Send>,
}

impl ImplLocationService {
    pub fn new(
        lng: f64,
        lat: f64,
        distance: f64,
        direction_type: DirectionType,
        new_angle_service: Box<dyn NewAngle + Send>,
        new_dest_service: Box<dyn NewDest + Send>,
        new_distance_service: Box<dyn NewDistance + Send>,
    ) -> Self {
        Self {
            lng,
            lat,
            distance,
            direction_type,
            angle: 0.0,
            new_angle_service,
            new_dest_service,
            new_distance_service,
        }
    }
}

impl LocationService for ImplLocationService {
    fn gen(&mut self) -> () {
        self.angle = self.new_angle_service.new_angle(self.direction_type);
        self.new_dest_service
            .new_dest(self.lng, self.lat, self.angle, self.distance);
    }
    fn get(&mut self) -> (f64, f64) {
        self.new_dest_service.get()
    }
    fn format(&mut self) -> String {
        self.new_dest_service.format()
    }
    fn concat(&mut self) -> String {
        self.new_dest_service.concat()
    }
    fn distance(&mut self, target_lng: f64, target_lat: f64) -> f64 {
        self.new_distance_service
            .distance(self.lng, self.lat, target_lng, target_lat)
    }
    fn detailed_direction(&mut self) -> String {
        self.new_angle_service.detailed_direction(self.angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::location::new_angle::MockNewAngle;
    use crate::service::location::new_dest::MockNewDest;
    use crate::service::location::new_distance::MockNewDistance;
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
            .return_const(()); // Always return () when called with these arguments

        mock_dest_service
            .expect_get()
            .with()
            .return_const((35.6761685462078, 140.87174397802116));

        let mut mock_distance_service = MockNewDistance::new();
        mock_distance_service
            .expect_distance()
            .with(
                eq(139.767125),
                eq(35.681236),
                eq(140.87174397802116),
                eq(35.6761685462078),
            )
            .return_const(100000.0);

        let mut location_service = ImplLocationService::new(
            139.767125,
            35.681236,
            100000.0,
            DirectionType::East,
            Box::new(mock_angle_service),
            Box::new(mock_dest_service),
            Box::new(mock_distance_service),
        );
        location_service.gen();
        let (lat, lng) = location_service.get();
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
        let formatted = location_service.format();
        assert_eq!(formatted, "+35.6761685462078+140.87174397802116");
        let concated = location_service.concat();
        assert_eq!(concated, "35.6761685462078,140.87174397802116");
        let distance = location_service.distance(140.87174397802116, 35.6761685462078);
        assert_eq!(distance, 100000.0);
    }
}
