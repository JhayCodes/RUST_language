/// A favorite color.
enum Color{
    Red, 
    Blue,
}

/// A piece of mail.
struct Mail{
    address: String,
}

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {}
 //to generate and open documentation, use cargo doc --open