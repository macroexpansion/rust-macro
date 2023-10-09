pub mod bounded;

#[macro_export]
macro_rules! a {
    ($name: ident, $inner_type: ty) => {
        pub struct $name {
            pub inner: $inner_type,
        }

        impl $name {
            pub fn new(inner: $inner_type) -> Self {
                Self { inner }
            }
        }
    };
}
