use geo::algorithm::geodesic_destination::GeodesicDestination;
use geo::Point;
use mockall::automock;

#[automock]
pub trait NewDest {
    fn new_dest(&self, lat: f64, lng: f64, angle: f64, distance: f64) -> (f64, f64);
}

pub struct NewDestService;

impl NewDest for NewDestService {
    /// Calculate a new geographic coordinate.
    ///
    /// # Arguments
    ///
    /// * `lat` - The latitude of the starting point.
    /// * `lng` - The longitude of the starting point.
    /// * `angle` - The angle of the direction to move, in degrees.
    /// * `distance` - The distance to move, in meters.
    ///
    /// # Returns
    ///
    /// * A tuple `(lat, lng)` representing the new geographic coordinate.
    fn new_dest(&self, lat: f64, lng: f64, angle: f64, distance: f64) -> (f64, f64) {
        let location = Point::new(lat, lng);
        let dest = location.geodesic_destination(angle, distance);
        (dest.y(), dest.x())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dest() {
        let service = NewDestService;
        let (lat, lng) = service.new_dest(139.767125, 35.681236, 90.0, 100000.0);
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
    }
}
