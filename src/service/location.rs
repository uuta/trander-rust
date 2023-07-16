use crate::service::location::new_dest::{NewDest, NewDestService};
use crate::util::rand_in_range;
use mockall::automock;

pub mod new_dest;

#[automock]
pub trait LocationService {
    fn location(&self) -> (f64, f64);
}

/// lat: latitute
/// lng: longitude
/// angle: angle of direction
/// distance: distance from the point (1km = 1000.0)
pub struct ImplLocationService {
    lng: f64,
    lat: f64,
    angle: f64,
    distance: f64,
    min: f64,
    max: f64,
    new_distance: fn(&Self) -> f64,
}

impl ImplLocationService {
    pub fn new(lng: f64, lat: f64, angle: f64, distance: f64, min: f64, max: f64) -> Self {
        Self {
            lng,
            lat,
            angle,
            distance,
            min,
            max,
            new_distance: Self::new_distance,
        }
    }
    fn new_distance(&self) -> f64 {
        rand_in_range(self.min, self.max)
    }
}

impl LocationService for ImplLocationService {
    fn location(&self) -> (f64, f64) {
        let new_dest_service = NewDestService;
        new_dest_service.new_dest(self.lng, self.lat, self.angle, self.distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location() {
        let location_service =
            ImplLocationService::new(139.767125, 35.681236, 90.0, 100000.0, 0.0, 1.0);
        let (lat, lng) = location_service.location();
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
    }

    #[test]
    fn test_new_distance() {
        let location_service =
            ImplLocationService::new(139.767125, 35.681236, 90.0, 100000.0, 0.0, 1.0);
        let new_distance = location_service.new_distance();
        assert!(new_distance >= 0.0 && new_distance <= 1.0);
    }
}
