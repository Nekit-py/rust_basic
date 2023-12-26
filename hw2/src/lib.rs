// Разделить код на модули исходя из логики элементов и их взаимосвязей. Протестировать код.

// Требования:
// Код должен быть логически корректно разделён на модули.
// Каждая функция протестирована.
// cargo clippy и cargo fmt --check не выдают ошибок и предупреждений.

#![allow(dead_code)]
mod counters;

mod pairs;

mod vec3tors;

#[cfg(test)]
mod tests {
    use crate::{
        counters::{signed, unsigned},
        pairs, vec3tors,
    };

    #[test]
    fn test_default_pair() {
        assert_eq!(pairs::default_pair(), (0, 0));
    }

    #[test]
    fn test_pair_vector_sum() {
        let a = (1, 2);
        let b = (2, 3);
        assert_eq!(pairs::pair_vector_sum(a, b), (3, 5));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let a = (1, 2);
        let b = (2, 3);
        assert_eq!(pairs::pair_scalar_sum(a, b), 8);
    }

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

    #[test]
    fn test_default_vec3() {
        assert_eq!(vec3tors::default_vec3(), [0; 3]);
    }

    #[test]
    fn test_vec3_vector_sum() {
        let a: vec3tors::Vec3 = [1; 3];
        let b: vec3tors::Vec3 = [2; 3];
        assert_eq!(vec3tors::vec3_vector_sum(a, b), [3; 3]);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let a: vec3tors::Vec3 = [1; 3];
        let b: vec3tors::Vec3 = [2; 3];
        assert_eq!(vec3tors::vec3_scalar_sum(a, b), 9);
    }
}
