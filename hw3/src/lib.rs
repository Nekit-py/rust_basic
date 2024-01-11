
pub fn first_or_second(tup: &mut(i32, i32), condition: bool) -> &mut i32 {
    // Принимает мутабельную ссылку на кортеж и bool значение.
    // Если false, возвращает мутабельную ссылку на первый элемент кортежа.
    // Если true, возвращает мутабельную ссылку на второй элемент кортежа.
    match condition {
        true => &mut tup.0,
        _ => &mut tup.1,
    }
}

pub fn elem_mut_ref_by_idx<T>(slice: &mut [T], n: usize) -> &mut T {
    // Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
    &mut slice[n]
}

pub fn elem_mut_ref_by_idx_end<T>(slice: &mut [T], n: usize) -> &mut T {
    // Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
    &mut slice[slice.len() - n]
}

pub fn two_slices<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
    // Принимает слайс и число N. Возвращает два слайса с элементами:
    // с нулевого по N-1;
    // с N-го по последний;
    (&slice[0..n], &slice[n..])

}

pub fn slice_array<T>(slice: &[T]) -> [&[T]; 4] {
    // Принимает слайс и возвращает массив слайсов,
    // содержащий четыре равные (насколько возможно) части исходного слайса.
    todo!();

}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_first_or_second() {
        let mut tup = (32, 33);
        assert_eq!(*first_or_second(&mut tup, true), 32);
        assert_eq!(*first_or_second(&mut tup, false), 33);
    }

    #[test]
    fn test_elem_mut_ref_by_idx() {
        let mut list = vec!["321", "123", "444"];
        assert_eq!(*elem_mut_ref_by_idx(&mut list[..], 2), "444");
    }

    #[test]
    #[should_panic]
    fn test_panic_elem_mut_ref_by_idx() {
        let mut list = vec![11, 22, 33];
        assert_eq!(*elem_mut_ref_by_idx(&mut list[..], 99999999), 33);
    }

    #[test]
    fn test_elem_mut_ref_by_idx_end() {
        let mut list = vec!['a', '3', '!'];
        assert_eq!(*elem_mut_ref_by_idx_end(&mut list[..], 2), '3');
    }

    #[test]
    #[should_panic]
    fn test_panic_elem_mut_ref_by_idx_end() {
        let mut list = vec![0.33333, 1243.321, 8.567312];
        assert_eq!(*elem_mut_ref_by_idx_end(&mut list[..], 99999999), 8.567312);
    }

    #[test]
    fn test_two_slices() {
        let vec = (0..10).collect::<Vec<u32>>();
        let (first, second) = two_slices(&vec, 3);
        assert_eq!(first, &[0,1,2]);
        assert_eq!(second, &[3,4,5,6,7,8,9]);
    }

    // #[test]
    // #[should_panic]
    // fn test_panic_two_slices() {
    //     let vec = (0..22).collect::<Vec<u32>>();
    //     let (first, second) = two_slices(&vec, 22);
    //     assert_eq!(first, &[4,5,6,7,8,9]);
    //     assert_eq!(second, &[0,1,2]);
    // }
}
