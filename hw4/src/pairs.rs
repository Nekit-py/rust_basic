use std::{
    ops::{Add, Sub},
    cmp::PartialEq
};



#[derive(Default, Debug)]
pub struct Pair(i32, i32);


impl Pair {
    pub fn scalar_sum(self, other: Self) -> i32 {
        self.0 + self.1 + other.0 + other.1
    }
}


impl Add for Pair {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Pair {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_default_pair() {
        assert_eq!(Pair::default(), Pair(0, 0));
    }

    #[test]
    fn test_pair_vector_sum() {
        let a = Pair(1, 2);
        let b = Pair(2, 3);
        assert_eq!(a + b, Pair(3, 5));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let a = Pair(1, 2);
        let b = Pair(2, 3);
        assert_eq!(a.scalar_sum(b), 8);
    }
}
