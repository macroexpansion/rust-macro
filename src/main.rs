use rust_macro::{a, bounded_impl};

a!(Test, String, a + b c + d);

trait Bounded {
    fn max_value() -> Self;
    fn min_value() -> Self;
}

bounded_impl!(u8, u8::MIN, u8::MAX);

fn main() {
    println!("Hello, world!");

    println!("{value}", value = u8::min_value());
    println!("{value}", value = u8::max_value());

    let test = Test::new("test string".to_string());
    println!("{}", test.inner);
    println!("{}", test.a);
    println!("{}", test.b);
    println!("{}", test.c);
    println!("{}", test.d);
}
