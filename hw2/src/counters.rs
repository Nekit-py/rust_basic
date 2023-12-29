pub mod signed {
    pub type SignedCounter = isize;

    pub fn default_signed_counter() -> SignedCounter {
        0
    }

    pub fn next_signed(counter: SignedCounter) -> SignedCounter {
        counter + 1
    }

    pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
        counter - 1
    }
}

pub mod unsigned {
    pub type UnsignedCounter = usize;

    pub fn default_unsigned_counter() -> UnsignedCounter {
        0
    }

    pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
        counter + 1
    }
}

#[cfg(test)]
pub mod tests {
    use super::{signed, unsigned};

    #[test]
    fn test_default_signed_counter() {
        assert_ne!(signed::default_signed_counter(), 1);
    }

    #[test]
    fn test_next_signed() {
        assert_eq!(signed::next_signed(1), 2);
    }

    #[test]
    fn test_prev_signed() {
        assert_eq!(signed::prev_signed(2), 1);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(unsigned::default_unsigned_counter(), 0);
    }

    #[test]
    fn test_next_unsigned() {
        assert_eq!(unsigned::next_unsigned(3), 4);
    }
}
