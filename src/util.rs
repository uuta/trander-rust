use rand::distributions::Uniform;
use rand::Rng;

/// returns random number in range [a, b]
pub fn rand_in_range(a: f64, b: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(a, b);
    rng.sample(range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_in_range() {
        let rand = rand_in_range(0.0, 1.0);
        assert!(0.0 <= rand && rand <= 1.0);
    }
}
