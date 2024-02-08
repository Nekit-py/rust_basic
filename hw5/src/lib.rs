/*
- Перенести все общие операции в трейт.
- Методы sum и is_default реализовать только один раз - в самом трейте.
- Реализовать трейт на обоих типах.
- Написать обобщённую тестовую логику, которая будет работать для обоих типов.
- Проверить корректность работы методов обоих типов используя общую тстовую логику.
Требования:
Все методы типов реализованы за счёт трейта.
Методы sum и is_default реализованы только один раз - в трейте.
Тесты для обоих типов используют общую тестовую логику.
Весь функционал каждого типа протестирован.
""cargo clippy"" и ""cargo fmt --check"" не выдают предупреждений и ошибок.
 */

pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

trait Sequence: Default {
    fn is_default(&self) -> bool;
    fn sum(&self) -> f64;
}

trait Accessor {
    fn get(&self, index: Item) -> f64;
    fn set(&mut self, index: Item, value: f64);
}

#[derive(Default)]
pub struct Tuple(u32, f32, f64);

impl Sequence for Tuple {
    fn is_default(&self) -> bool {
        let default = Self::default();
        self.0 == default.0 && self.1 == default.1 && self.2 == default.2
    }

    fn sum(&self) -> f64 {
        self.0 as f64 + self.1 as f64 + self.2
    }
}

impl Accessor for Tuple {
    fn get(&self, index: Item) -> f64 {
        match index {
            Item::First => self.0 as f64,
            Item::Second => self.1 as f64,
            Item::Third => self.2,
        }
    }

    fn set(&mut self, index: Item, value: f64) {
        match index {
            Item::First => self.0 = value as u32,
            Item::Second => self.1 = value as f32,
            Item::Third => self.2 = value,
        }
    }
}

#[derive(Default)]
pub struct Array([f64; 3]);

impl Sequence for Array {
    fn sum(&self) -> f64 {
        self.0.iter().sum()
    }

    fn is_default(&self) -> bool {
        self.0.iter().all(|&value| value == 0.0)
    }
}

impl Accessor for Array {
    fn get(&self, index: Item) -> f64 {
        let idx = index.index();
        self.0[idx]
    }

    fn set(&mut self, index: Item, value: f64) {
        self.0[index.index()] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_sum() {
        let tuple = Tuple(3, 4.0, 5.0);
        assert_eq!(tuple.sum(), 12.0);

        let array = Array([3.0, 4.0, 5.0]);
        assert_eq!(array.sum(), 12.0);
    }

    #[test]
    fn test_sequence_is_default() {
        let tuple = Tuple(0, 0.0, 0.0);
        assert!(tuple.is_default());

        let array = Array([0.0, 0.0, 0.0]);
        assert!(array.is_default());

        let non_default_tuple = Tuple(1, 0.0, 0.0);
        assert!(!non_default_tuple.is_default());

        let non_default_array = Array([1.0, 0.0, 0.0]);
        assert!(!non_default_array.is_default());
    }

    #[test]
    fn test_accessor_get() {
        let tuple = Tuple(1, 2.2, 3.3);
        assert_eq!(tuple.get(Item::Third), 3.3);

        let array = Array([1.0, 2.0, 3.0]);
        assert_eq!(array.get(Item::Third), 3.0);
    }

    #[test]
    fn test_accessor_set() {
        let mut tuple = Tuple(1, 2.2, 3.3);
        tuple.set(Item::First, 111.0);
        assert_eq!(tuple.0, 111);

        let mut array = Array([1.0, 2.0, 3.0]);
        array.set(Item::Second, 4.44);
        assert_eq!(array.get(Item::Second,), 4.44);
    }
}
