// Function to find a generator for small-size integers
pub fn find_generator(prime: usize) -> usize {
    for i in 2..prime {
        if is_generator(prime, i) {
            return i
        }
    }
    0
}

fn is_generator(prime: usize, g: usize) -> bool {
    let mut val = g;
    for _ in 2..prime-1 {
        val = val*g % prime;
        if val == 1 {
            return false
        }
    }

    val*g % prime == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_generator() {
        assert!(is_generator(11, 2));
        
        assert!(is_generator(17, 3));
        assert!(!is_generator(17, 2));

        // All roots of F29
        assert!(is_generator(29, 2));
        assert!(is_generator(29, 3));
        assert!(is_generator(29, 8));
        assert!(is_generator(29, 10));
        assert!(is_generator(29, 11));
        assert!(is_generator(29, 14));
        assert!(is_generator(29, 15));
        assert!(is_generator(29, 18));
        assert!(is_generator(29, 19));
        assert!(is_generator(29, 21));
        assert!(is_generator(29, 26));
        assert!(is_generator(29, 27));

        // All non-roots of F29
        assert!(!is_generator(29, 4));
        assert!(!is_generator(29, 5));
        assert!(!is_generator(29, 6));
        assert!(!is_generator(29, 7));
        assert!(!is_generator(29, 9));
        assert!(!is_generator(29, 12));
        assert!(!is_generator(29, 13));
        assert!(!is_generator(29, 16));
        assert!(!is_generator(29, 17));
        assert!(!is_generator(29, 20));
        assert!(!is_generator(29, 22));
        assert!(!is_generator(29, 23));
        assert!(!is_generator(29, 24));
        assert!(!is_generator(29, 25));
        assert!(!is_generator(29, 28));
    }

    #[test]
    fn test_find_generator() {
        // Answers to the homework!
        assert_eq!(find_generator(1009), 11);
        assert_eq!(find_generator(2357), 2);
        
        // What happens if we don't pass a prime? We simply 0
        assert_eq!(find_generator(15), 0);
        assert_eq!(find_generator(100), 0);
    }
}
