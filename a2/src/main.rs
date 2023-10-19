//display the result of the sum of two numbers
//sum function
fn sum(a:i32, b:i32) -> i32{
    a + b
}

fn display_result(result:i32){
    println!("{:?}", result);
}

fn main() {
    let result = sum(5,9);
    display_result(result);
}
