#[macro_export]
macro_rules! assert_that {
    ($a:expr; equals $e:expr) => (assert_eq!($e, $a, "{} is {} but expected to be {}", stringify!($a), $a, $e, ));
}
