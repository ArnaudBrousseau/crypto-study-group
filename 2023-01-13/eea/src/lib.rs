/// Extended Euclidean Algorithm
/// Returns GCD(a,b) and the pair (u, v) such that:
///     au + bv = GCD(a, b)
///     (Bezout identity)
pub fn eea(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 || b == 0 {
        return (-1, -1, -1);
    }
    
    if a > b {
        compute_eea(b, a, 1, 0, 0, 1)
    } else {
        compute_eea(a, b, 1, 0, 0, 1)
    }
}

/// Recursive inner function
/// For each round, a < b, and we iterate on a, b, ua/va, and ub/vb
fn compute_eea(a: i32, b: i32, ua: i32, va: i32, ub: i32, vb: i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, ub, vb)
    }
    let quotient = b/a;
    let remainder = b - quotient*a;

    compute_eea(remainder, a, ub-quotient*ua, vb-quotient*va, ua, va)
}

#[cfg(test)]
mod test {
    use crate::eea;

    #[test]
    fn test_eea() {
        assert_eq!(eea(0, 0), (-1, -1, -1));
        assert_eq!(eea(0, 5), (-1, -1, -1));
        assert_eq!(eea(10, 10), (10, 1, 0));
        
        // Calculated manually
        assert_eq!(eea(397, 2357), (1, -754, 127));
        // Ordering shouldn't matter
        assert_eq!(eea(2357, 397), (1, -754, 127));
    }
}
