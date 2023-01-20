/// Simple, inefficient function to test primality.
/// Only suitable for small integers
pub fn is_prime(n: u32) -> bool {
    if n == 1 || n == 2 {
        return true;
    } else if n > 2 {
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::is_prime;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(45));
        assert!(!is_prime(46));
        assert!(is_prime(47));
        assert!(is_prime(397));
        assert!(is_prime(2357));
    }
}
