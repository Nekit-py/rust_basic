
fn double_int32(num: i32) ->i32 {
    num << 1
}

fn double_int64(num: i32) -> i64 {
    (num as i64) << 1
}

fn double_float32(num: f32) -> f32 {
    num * 2f32
}

fn double_float64(num: f32) -> f64 {
    (num  as f64)* 2_f64
}

fn int_plus_float_to_float(i: i32, f: f32) -> f64 {
    i as f64 + f as f64
}

fn int_plus_float_to_int(i: i32, f: f32) -> i64 {
    i as i64 + f as i64 
}

fn tuple_sum(nums: (i32, i32)) -> i32 {
    let (num_1, num_2) = nums;
    num_1.saturating_add(num_2)
}

fn array_sum(arr: [i32; 3]) -> i32 {
    arr.into_iter().sum()
}

fn main() {
    println!("{:?}", double_int32(-4));
    println!("{:?}", double_int32(32));

    println!("{:?}", double_int64(100));
    println!("{:?}", double_int64(2000_000_000));

    println!("{:?}", double_float32(3.33));
    println!("{:?}", double_float32(214748364.12123445));

    println!("{:?}", double_float64(3.45647128394));
    println!("{}", int_plus_float_to_float(-3, 5.83489));
    println!("{}", int_plus_float_to_int(111111, -9.22222));
    println!("{}", tuple_sum((123, 2147483647)));
    println!("{:?}", array_sum([47483647,22,89]));
}
