use std::cmp::PartialEq;

#[derive(Debug, Default)]
pub struct SignedCounter {
    pub value: isize,
}

impl PartialEq for SignedCounter {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl SignedCounter {
    pub fn next(&mut self) {
        self.value += 1;
    }

    pub fn prev(&mut self) {
        self.value -= 1;
    }
}

#[derive(Debug, Default)]
pub struct UnsignedCounter {
    pub value: usize,
}

impl UnsignedCounter {
    pub fn next(&mut self) {
        self.value += 1;
    }
}

#[cfg(test)]
pub mod tests {
    use super::{SignedCounter, UnsignedCounter};

    #[test]
    fn test_default_signed_counter() {
        assert_eq!(SignedCounter::default(), SignedCounter { value: 0 });
    }

    #[test]
    fn test_next_signed() {
        let mut sc = SignedCounter::default();
        sc.next();
        assert_eq!(sc.value, 1);
    }

    #[test]
    fn test_prev_signed() {
        let mut sc = SignedCounter::default();
        sc.prev();
        assert_eq!(sc.value, -1);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(UnsignedCounter::default().value, 0);
    }

    #[test]
    fn test_next_unsigned() {
        let mut uc = UnsignedCounter::default();
        uc.next();
        assert_eq!(uc.value, 1);
    }
}
