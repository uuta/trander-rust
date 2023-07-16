use crate::service::location::new_dest::{NewDest, NewDestService};
use mockall::automock;

pub mod new_dest;
pub mod new_distance;

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
}

impl ImplLocationService {
    pub fn new(lng: f64, lat: f64, angle: f64, distance: f64) -> Self {
        Self {
            lng,
            lat,
            angle,
            distance,
        }
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
        let location_service = ImplLocationService::new(139.767125, 35.681236, 90.0, 100000.0);
        let (lat, lng) = location_service.location();
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
    }
}
