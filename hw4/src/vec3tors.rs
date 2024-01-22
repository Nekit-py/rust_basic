use std::ops::Add;
use std::iter::zip;


#[derive(Debug)]
pub struct Vec3 {
    pub value: [i32; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Self{value:[0;3]}
    }
}

impl Add for Vec3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        let mut value = self.value;
        for i in 0..3 {
            value[i] += other.value[i];
        }
        Self { value }
    }
}

impl Vec3 {
    pub fn new(arr: [i32;3]) -> Vec3 {
        Vec3{value: arr}
    }

    pub fn scalar_sum(&self, other: Self) -> i32 {
       zip(self.value, other.value)
        .map(|(x,y)| x+y)
        .sum()
    }
}

// pub const VEC3_LEN: usize = 3;

// pub type Vec3 = [i32; VEC3_LEN];

// pub fn default_vec3() -> Vec3 {
//     [0; 3]
// }

// pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
//     let mut c = default_vec3();
//     for i in 0..3 {
//         c[i] = a[i] + b[i];
//     }
//     c
// }

// pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
//     let mut c = 0;
//     for i in 0..VEC3_LEN {
//         c += a[i] + b[i];
//     }
//     c
// }

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
