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

trait Sequence: Default + Accessor {
    fn is_default(&self) -> bool {
        let default = Self::default();
        if default.get(Item::First) == self.get(Item::First)
            && default.get(Item::Second) == self.get(Item::Second)
            && default.get(Item::Third) == self.get(Item::Third)
        {
            return true;
        }
        false
    }

    fn sum(&self) -> f64 {
        self.get(Item::First) + self.get(Item::Second) + self.get(Item::Third)
    }
}

trait Accessor {
    fn get(&self, index: Item) -> f64;
    fn set(&mut self, index: Item, value: f64);
}

#[derive(Default)]
pub struct Tuple(u32, f32, f64);

impl Sequence for Tuple {}

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

impl Sequence for Array {}

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

    fn test_sequence<T: Sequence>() {
        let mut sequence = T::default();

        // Test default values
        assert!(sequence.is_default());

        // Test setting and getting values (Accessor)
        sequence.set(Item::First, 1.0);
        sequence.set(Item::Second, 2.0);
        sequence.set(Item::Third, 3.0);
        assert_eq!(sequence.get(Item::First), 1.0);
        assert_eq!(sequence.get(Item::Second), 2.0);
        assert_eq!(sequence.get(Item::Third), 3.0);

        // Test sum
        assert_eq!(sequence.sum(), 6.0);

        // Test non-default values
        assert!(!sequence.is_default());
    }

    #[test]
    fn test_tuple_sequence() {
        test_sequence::<Tuple>();
    }

    #[test]
    fn test_array_sequence() {
        test_sequence::<Array>();
    }
}
