use rust_macro::bounded_impl;

trait Bounded {
    fn max_value() -> Self;
    fn min_value() -> Self;
}

bounded_impl!(u8, u8::MIN, u8::MAX);

fn main() {
    println!("Hello, world!");

    println!("{value}", value = u8::min_value());
    println!("{value}", value = u8::max_value());
}
