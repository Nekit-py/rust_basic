#![allow(dead_code)]
use std::collections::HashMap;

/*
    Написать функцию, которая превращает число в строку по следующим правилам:
    1. Если число кратно 3, то возвращаем строку "Fizz"
    2. Если число кратно 5, то возвращаем строку "Buzz"
    3. Если число кратно и 3, и 5, то возвращаем строку "FizzBuzz"
    4. В остальных случаях возвращаем строку, содержащую данное число
    
    Написать функцию fizzbuzz_list, которая получает число `n: u32` и возвращает
    список строк, содержащих строковые представления fizzbuzz
    для чисел в диапазоне от 1 до n. Написать тесты.
*/
fn fizzbuzz(num: u32) -> String {
    let string: String = if num % 3 == 0 {
        if num % 5 == 0{
            "FizzBuzz".to_owned()
        } else {
        "Fizz".to_owned()
        }
    } else if num % 5 == 0 {
        "Buzz".to_owned()
    } else {
        num.to_string()
    };
    string
}


/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при это цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

fn digit_product(n: u32) -> u8 {
    let n_string = n.to_string();
    if n_string.len() == 1 {
        return n as u8
    }

    let mut new_n = 1;
    for ch in n_string.chars() {
        if ch == '0' {continue}
        new_n *= ch.to_digit(10).unwrap();
    }
    if new_n < 10 {return new_n as u8}
    digit_product(new_n)
}

/*
    Последовательностью Фибоначчи называется последовательность чисел,
    которая удовлетворяет следующим условиям:
    - элемент последовательности с индексом 0 - число 0
    - элемент с индексом 1 - число 1
    - каждый последующий элемент равен сумме двух предыдущих.

    0, 1, 1, 2, 3, 5, 8, 13, 21 ...

    Написать функцию, которая вычислит элемент последовательности с индексом n.

    * Написать вторую функцию, которая вернёт последовательность Фибонначи
      от первого элемента до n-ого. Написать тесты
*/

fn fib(n: u32) -> u32 {
    if n == 1 || n == 0 {return n}
    let mut prev = 0u32;
    let mut cur = 1u32;
    let mut summ: u32 = 0;
    let mut idx = 2;
    while idx <= n {
        summ = prev + cur;
        prev = cur;
        cur = summ;
        idx += 1;
    }
    summ
}

/*
    Дана строка, состоящая только из цифровых символов. В данной строке
    есть одна цифра, которая не повторяется. Написать функцию,
    которая найдёт эту цифру и вернёт её.

    * Написать похожую функцию, но только на этот раз в данной строке
    могут присутствовать любые символы, а уникальная цифра может отсутствовать.
    Но если присутсвует, то не больше одной. Написать тесты.
*/


fn uniq_digit(s: &str) -> u8 {
    if s.len() == 1 {return s.parse::<u8>().unwrap()}
    let mut map : HashMap<char, u8>= HashMap::new();
       for ch in s.chars() {
               map.entry(ch).and_modify(|counter| *counter += 1)
                   .or_insert(1);
       }
    let mut result: u8 = 0;
    for (k, v) in map.iter() {
        if *v == 1 {
            result = k.to_digit(10).unwrap() as u8
        }
    }
   result
}

fn uniq_digit_2(s: &str) -> Option<u8> {
    if s.len() == 1 {
        match s.parse::<u8>().ok(){
           Some(num) => return Some(num),
           None => return None 
        }
        // return s.parse::<u8>().unwrap()
    }
    let mut map : HashMap<char, u8>= HashMap::new();
       for ch in s.chars() {
            if !ch.is_ascii_digit() {continue}
               map.entry(ch).and_modify(|counter| *counter += 1)
                   .or_insert(1);
       }
    let mut result: Option<u8> = None;
    for (k, v) in map.iter() {
        if *v == 1 {
            result = Some(k.to_digit(10).unwrap() as u8)
        }
    }
   result
}

/*
    Дан массив, который содержит n неповторяющихся чисел в диапазоне
    от 0 до n включительно.

    Написать функцию, которая вернёт единственное число, отсутствующее
    в данном массиве.
    
    Гарантируется, что числа в массиве не повторяются и все принадлежат
    заданному диапазону.
*/

fn missing_num(nums: &[i32]) -> i32 {
    let a = (0..=nums.len() as i32).sum::<i32>();
    let b = nums.iter().sum::<i32>();
    a - b
}

/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара
    
    Написать функцию, которая проверит корректность данной строки.
*/

fn validate_parent(s: &str) -> bool {
    let map = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);

    let mut counter : HashMap<char, u8>= HashMap::new();

    for ch in s.chars() {
            counter.entry(ch).and_modify(|counter| *counter += 1)
                .or_insert(1);
    }

    for (k, v) in counter.iter() {
        let pair = map.get(k).unwrap();
        let count = counter.get(pair);

        match count {
           Some(num) => {
            if v != num { return false }
           },
           None => return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }
    fn test_validate_parent() {
        assert_eq!(validate_parent("()"), true);
        assert_eq!(validate_parent("()[]{}"), true);
        assert_eq!(validate_parent("({[]()})"), true);
        assert_eq!(validate_parent("(}"), false);
        assert_eq!(validate_parent("()]"), false);
        assert_eq!(validate_parent("(){"), false);
    }

    #[test]
    fn test_digit_product() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2);
        assert_eq!(digit_product(123456), 4);
        assert_eq!(digit_product(123454321), 6);
    }
    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(7), 13);
    }
    #[test]
    fn test_uniq_digit() {
        assert_eq!(uniq_digit("3"), 3);
        assert_eq!(uniq_digit("010"), 1);
        assert_eq!(uniq_digit("47343077"), 0);
        assert_eq!(uniq_digit("123454321"), 5);
        assert_eq!(uniq_digit("0987654321234567890"), 1);
        assert_eq!(uniq_digit("4444444444424444444444444"), 2);
    }

    #[test]
    fn test_uniq_digit_2() {
        assert_eq!(uniq_digit_2("3"), Some(3));
        assert_eq!(uniq_digit_2("010"), Some(1));
        assert_eq!(uniq_digit_2("47343077"), Some(0));
        assert_eq!(uniq_digit_2("123454321"), Some(5));
        assert_eq!(uniq_digit_2("0987654321234567890"), Some(1));
        assert_eq!(uniq_digit_2("4444444444424444444444444"), Some(2));
        assert_eq!(uniq_digit_2("fsahjkhui1...{}#$"), Some(1));
    }
    #[test]
    fn test_missing_num() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}