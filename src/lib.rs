pub mod bounded;

#[macro_export]
macro_rules! a {
    ($name:ident, $inner_type:ty, $($a:ident + $b:ident)*) => {
        #[derive(Default)]
        pub struct $name {
            pub inner: $inner_type,
            $(
            pub $a: usize,
            pub $b: usize,
            )*
            }

            impl $name {
        pub fn new(inner: $inner_type) -> Self {
        Self { inner, ..Default::default() }
        }
        }
    };
}
