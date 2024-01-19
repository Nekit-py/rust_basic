
pub struct SignedCounter {
    pub value: isize
}

impl Default for SignedCounter {
    fn default() -> Self {
        SignedCounter { value: 0}
    }
}

impl SignedCounter {
    pub fn next(&mut self) {
        self.value +=1;
    }

    pub fn prev(&mut self) {
        self.value -=1;
    }
}


pub struct UnsignedCounter {
    pub value: usize
}

impl Default for UnsignedCounter {
    fn default() -> Self {
        UnsignedCounter { value: 0}
    }
}

impl UnsignedCounter {
    pub fn next(&mut self) {
        self.value += 1;
    }
}

#[cfg(test)]
pub mod tests {
    // use super::{signed, unsigned};
    use super::{SignedCounter, UnsignedCounter};

    #[test]
    fn test_default_signed_counter() {
        assert_eq!(SignedCounter::default(), 0);
    }

    #[test]
    fn test_next_signed() {
        let mut sc = SignedCounter::default();
        assert_eq!(sc::next(), 1);
    }

    #[test]
    fn test_prev_signed() {
        let mut sc = SignedCounter::default();
        assert_eq!(sc::prev(), -1);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(UnsignedCounter::default(), 0);
    }

    #[test]
    fn test_next_unsigned() {
        let mut uc = UnsignedCounter::default():
        assert_eq!(uc::next(), 1);
    }
}
