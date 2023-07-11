use geo::algorithm::geodesic_destination::GeodesicDestination;
use geo::Point;
use mockall::automock;

#[automock]
pub trait LocationService {
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
}

impl ImplLocationService {
    pub fn new(lat: f64, lng: f64, angle: f64, distance: f64) -> Self {
        Self {
            lat,
            lng,
            angle,
            distance,
        }
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
        let location_service = ImplLocationService::new(35.681236, 139.767125, 90.0, 100000.0);
        let (lat, lng) = location_service.new_dest();
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
    }
}
