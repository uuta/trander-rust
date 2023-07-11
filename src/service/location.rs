use crate::util::rand_in_range;
use geo::algorithm::geodesic_destination::GeodesicDestination;
use geo::Point;
use mockall::automock;

#[automock]
pub trait LocationService {
    /// TODO: Change new_dest to private
    fn new_dest(&self) -> (f64, f64);
}

/// lat: latitute
/// lng: longitude
/// angle: angle of direction
/// distance: distance from the point (1km = 1000.0)
pub struct ImplLocationService {
    lat: f64,
    lng: f64,
    angle: f64,
    distance: f64,
    min: f64,
    max: f64,
    new_distance: fn(&Self) -> f64,
}

impl ImplLocationService {
    pub fn new(lat: f64, lng: f64, angle: f64, distance: f64, min: f64, max: f64) -> Self {
        Self {
            lat,
            lng,
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
    fn new_dest(&self) -> (f64, f64) {
        let location = Point::new(self.lng, self.lat);
        let dest = location.geodesic_destination(self.angle, self.distance);
        (dest.y(), dest.x())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_service() {
        let location_service =
            ImplLocationService::new(35.681236, 139.767125, 90.0, 100000.0, 0.0, 1.0);
        let (lat, lng) = location_service.new_dest();
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
    }

    #[test]
    fn test_new_distance() {
        let location_service =
            ImplLocationService::new(35.681236, 139.767125, 90.0, 100000.0, 0.0, 1.0);
        let new_distance = location_service.new_distance();
        assert!(new_distance >= 0.0 && new_distance <= 1.0);
    }
}
