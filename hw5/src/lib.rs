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

use std::ops::{Index, Add, Sub, Mul, Div, IndexMut};

pub enum Item {
    First,
    Second,
    Third,
}

// По сути тут я хотел убить 3 зайцев сразу:
// 1) Чтобы к элементам типа Tuple и Array я мог обращаться по индексу,
// указывая вместо числа Item
// 2) Метод get_item я хотел заменить просто обращением по индексу,
// получая ссылку на нужные элемент коллекции
// 3) Метод set_item - на IndexMut. По мутабельной ссылке присваивать значение,
// но не удалось, т.к. кортеж содерижт разные типы.
// Если в трейте Index я могу просто скастить в нужный тип, то IndexMut уже такое не допускает и это логично,
// не придумал как обойти...
// 4) Метод sum в трейте Sumary я хотел реализовать просто суммой 3 элементов коллекий (Tuple & Array), т.к. у обоих фиксированная длина.
// Аналогичным образом is_default, через индекс Item.
// Где я свернул не туда?)


impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

#[derive(Debug, Default)]
pub struct Tuple(u32, f32, f64);

impl Index<Item> for Tuple {
    type Output = f64;

    fn index(&self, item: Item) -> &Self::Output {
        match item {
            Item::First => &(self.0 as f64),
            Item::Second => &(self.1 as f64),
            Item::Third => &self.2,
        }
    }
}

impl IndexMut<Item> for Tuple {

    fn index_mut(&mut self, item: Item) -> &mut Self::Output {
        match item {
            Item::First => &mut self.0,
            Item::Second => &mut self.1,
            Item::Third => &mut self.2,
        }
    }
}

impl Tuple {

    pub fn is_default(&self) -> bool {
        self.0 == 0 && self.1 == 0.0 && self.2 == 0.0
    }

    pub fn sum(&self) -> f64 {
        self.0 as f64 + self.1 as f64 + self.2
    }
}

#[derive(Debug, Default)]
pub struct Array<T: Add + Sub + Mul + Div>([T; 3]);

impl<T: Add + Sub + Mul + Div> Index<Item> for Array<T> {
    type Output = T;

    fn index(&self, item: Item) -> &Self::Output {
        match item {
            Item::First => &self.0[0],
            Item::Second => &self.0[1],
            Item::Third => &self.0[2],
        }
    }
}

impl<T: Add + Sub + Mul + Div> IndexMut<Item> for Array<T> {

    fn index_mut(&mut self, item: Item) -> &mut Self::Output {
        match item {
            Item::First => &mut self.0[0],
            Item::Second => &mut self.0[1],
            Item::Third => &mut self.0[2],
        }
    }
}

// impl Array {
//     pub fn is_default(&self) -> bool {
//         for value in &self.0 {
//             if *value != 0.0 {
//                 return false;
//             }
//         }
//         true
//     }

//     pub fn sum(&self) -> f64 {
//         let mut sum = 0.0;
//         for value in &self.0 {
//             sum += *value;
//         }
//         sum
//     }
// }

trait Sumary {
    type Item = Item;

    fn sum(&self) -> f64 {
        &self.[Item::First] as f64 + &self.[Item::Second] as f64 + &self.[Item::Second] as f64
    }
}



// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
