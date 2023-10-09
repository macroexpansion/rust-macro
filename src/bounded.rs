#[macro_export]
macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }
            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}
