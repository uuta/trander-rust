use geo::algorithm::geodesic_destination::GeodesicDestination;
use geo::Point;
use mockall::automock;

#[automock]
pub trait NewDest {
    fn new_dest(&mut self, lat: f64, lng: f64, angle: f64, distance: f64);
    fn get(&self) -> (f64, f64);
    fn format(&self) -> String;
}

pub struct NewDestService {
    dest: Point<f64>,
}

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
    fn new_dest(&mut self, lat: f64, lng: f64, angle: f64, distance: f64) {
        let location = Point::new(lat, lng);
        self.dest = location.geodesic_destination(angle, distance);
    }

    /// Get the new geographic coordinate.
    ///
    /// # Returns
    ///
    /// * A tuple `(lat, lng)` representing the new geographic coordinate.
    fn get(&self) -> (f64, f64) {
        (self.dest.y(), self.dest.x())
    }

    fn format(&self) -> String {
        format!("{} {}", self.dest.y(), self.dest.x())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dest() {
        let mut service = NewDestService {
            dest: Point::new(0.0, 0.0),
        };
        service.new_dest(139.767125, 35.681236, 90.0, 100000.0);
        let (lat, lng) = service.get();
        assert_eq!(lat, 35.6761685462078);
        assert_eq!(lng, 140.87174397802116);
    }
}
