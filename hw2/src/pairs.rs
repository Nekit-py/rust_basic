pub type Pair = (i32, i32);

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_default_pair() {
        assert_eq!(default_pair(), (0, 0));
    }

    #[test]
    fn test_pair_vector_sum() {
        let a = (1, 2);
        let b = (2, 3);
        assert_eq!(pair_vector_sum(a, b), (3, 5));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let a = (1, 2);
        let b = (2, 3);
        assert_eq!(pair_scalar_sum(a, b), 8);
    }
}
