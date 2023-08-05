use crate::util::rand_in_range;
use geo::point;
use geo::prelude::*;
use mockall::automock;

#[automock]
pub trait NewDistance {
    fn new_distance(&self, min: f64, max: f64) -> f64;
    fn distance(&self, lng: f64, lat: f64, target_lng: f64, target_lat: f64) -> f64;
}

pub struct NewDistanceService;

impl NewDistance for NewDistanceService {
    /// Calculate a new distance.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum distance.
    /// * `max` - The maximum distance.
    ///
    /// # Returns
    ///
    /// * A new distance.
    fn new_distance(&self, min: f64, max: f64) -> f64 {
        rand_in_range(min, max)
    }

    /// Calculate the distance between two points.
    ///
    /// # Arguments
    ///
    /// * `lng` - The longitude of the first point.
    /// * `lat` - The latitude of the first point.
    /// * `target_lng` - The longitude of the second point.
    /// * `target_lat` - The latitude of the second point.
    ///
    /// # Returns
    ///
    /// * The distance between the two points.
    fn distance(&self, lng: f64, lat: f64, target_lng: f64, target_lat: f64) -> f64 {
        let p1 = point!(x: lng, y: lat);
        let p2 = point!(x: target_lng, y: target_lat);
        p1.haversine_distance(&p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_distance() {
        let service = NewDistanceService;
        let new_distance = service.new_distance(0.0, 1.0);
        assert!(new_distance >= 0.0 && new_distance <= 1.0);
    }

    #[test]
    fn test_distance() {
        let service = NewDistanceService;
        let distance = service.distance(-74.006f64, 40.7128f64, -0.1278f64, 51.5074f64);
        assert_eq!(
            5_570_230., // meters
            distance.round()
        );
    }
}
