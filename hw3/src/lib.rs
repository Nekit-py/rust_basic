/// Принимает мутабельную ссылку на кортеж и bool значение.
/// Если false, возвращает мутабельную ссылку на первый элемент кортежа.
/// Если true, возвращает мутабельную ссылку на второй элемент кортежа.
pub fn first_or_second<T>(tup: &mut (T, T), condition: bool) -> &mut T {
    match condition {
        true => &mut tup.0,
        false => &mut tup.1,
    }
}

/// Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
pub fn elem_mut_ref_by_idx<T>(slice: &mut [T], n: usize) -> &mut T {
    &mut slice[n]
}

/// Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
pub fn elem_mut_ref_by_idx_end<T>(slice: &mut [T], n: usize) -> &mut T {
    if n == 0 {
        &mut slice[slice.len() - 1]
    }
    else {
        &mut slice[slice.len() - n]
    }
}

/// Принимает слайс и число N. Возвращает два слайса с элементами:
/// с нулевого по N-1;
/// с N-го по последний;
pub fn two_slices<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
    (&slice[..n], &slice[n..])
}

/// Принимает слайс и возвращает массив слайсов,
/// содержащий четыре равные (насколько возможно) части исходного слайса.
pub fn slice_array<const N: usize, T>(slice: &[T]) -> [&[T]; N] {
    let mut result = [&slice[..]; N]; //[&slice[0..0]; 4]

    if N == 0 {
        return result;
    }

    let slice_len = slice.len();
    let chank_size = slice_len / N;
    let mut remainder = slice_len % N;
    let mut start = 0;

    for s in &mut result {
        let mut end = start + chank_size;

        if remainder > 0 {
            end += 1;
            remainder -= 1;
        }

        *s = &slice[start..end];
        start = end;
    }
    result
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
        let mut list = ["321", "123", "444"];
        assert_eq!(*elem_mut_ref_by_idx(&mut list[..], 2), "444");
    }

    #[test]
    #[should_panic]
    fn test_panic_elem_mut_ref_by_idx() {
        let mut list = [11, 22, 33];
        assert_eq!(*elem_mut_ref_by_idx(&mut list[..], 99999999), 33);
    }

    #[test]
    fn test_elem_mut_ref_by_idx_end() {
        let mut list = ['a', '3', '!'];
        assert_eq!(*elem_mut_ref_by_idx_end(&mut list[..], 2), '3');
    }

    #[test]
    #[should_panic]
    fn test_panic_elem_mut_ref_by_idx_end() {
        let mut list = [0.33333, 1243.321, 8.567312];
        assert_eq!(*elem_mut_ref_by_idx_end(&mut list[..], 99999999), 8.567312);
    }

    #[test]
    fn test_two_slices() {
        let vec = (0..10).collect::<Vec<u32>>();
        let (first, second) = two_slices(&vec, 3);
        assert_eq!(first, &[0, 1, 2]);
        assert_eq!(second, &[3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_slice_array() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(slice_array::<4, i32>(&array), [&[1, 2], &[3, 4], &[5, 6], &[7, 8]]);
    }

    #[test]
    #[should_panic]
    fn panic_test_slice_array() {
        let array = [1, 2, 3];
        assert_eq!(slice_array::<4, i32>(&array), [&[1, 2], &[3, 4], &[5, 6], &[7, 8]]);
    }
}
