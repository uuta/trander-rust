use crate::util::rand_in_range;
use mockall::automock;

#[automock]
pub trait NewDistance {
    fn new_distance(&self, min: f64, max: f64) -> f64;
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
}
