use std::iter::zip;
use std::ops::Add;

#[derive(Debug, Default)]
pub struct Vec3 {
    pub value: [i32; 3],
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut value = self.value;
        for idx in 0..3 {
            value[idx] += other.value[idx];
        }
        Self { value }
    }
}

impl Vec3 {
    pub fn new(arr: [i32; 3]) -> Vec3 {
        Vec3 { value: arr }
    }

    pub fn scalar_sum(&self, other: Self) -> i32 {
        zip(self.value, other.value).map(|(x, y)| x + y).sum()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_default_vec3() {
        assert_eq!(Vec3::default().value, [0; 3]);
    }

    #[test]
    fn test_vec3_vector_sum() {
        let a = Vec3::new([1; 3]);
        let b = Vec3::new([2; 3]);
        assert_eq!((a + b).value, [3; 3]);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let a = Vec3::new([1; 3]);
        let b = Vec3::new([2; 3]);
        assert_eq!(a.scalar_sum(b), 9);
    }
}
