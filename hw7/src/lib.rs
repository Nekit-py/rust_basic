#[macro_export]
macro_rules! my_macro {
    ($($func:ident),*) => {
        (
            $(
                $func(),
            )*
        )
    };
}

pub fn foo() -> i32 {
    1
}

pub fn bar() -> i32 {
    2
}

pub fn baz() -> i32 {
    3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!((1, 2, 3), my_macro!(foo, bar, baz));
    }
}
