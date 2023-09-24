use rand::distributions::Uniform;
use rand::Rng;

/// returns random number in range [a, b]
pub fn rand_in_range(a: f64, b: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(a, b);
    rng.sample(range)
}

/// returns random char
pub fn generate_random_char() -> char {
    let mut rng = rand::thread_rng();
    // Since 'a' is 97 and 'z' is 122 in ASCII values, it generates random numbers within this range.
    let random_number = rng.gen_range(97..=122);
    // Convert the number to char type.
    random_number as u8 as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_in_range() {
        let rand = rand_in_range(0.0, 1.0);
        assert!(0.0 <= rand && rand <= 1.0);
    }

    #[test]
    fn test_generate_random_char() {
        let rand = generate_random_char();
        assert!(rand.is_ascii_alphabetic());
    }
}
