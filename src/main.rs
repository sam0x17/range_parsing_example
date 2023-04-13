use range_example_macros::*;

fn main() {
    let range_str = example_parse!(44..123);
    println!("got: {}", range_str);
}
