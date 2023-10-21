fn sumation(a:i32, b:i32) -> i32{
    a + b
}

fn sub(a:i32, b:i32) -> i32{
    a - b
}

fn div(a:i32, b:i32) -> i32{
    a/b
}

fn main() {
    let sum = 2 + 2; //4
    let value = 10 - 5; //5
    let division = 10/2; //8
    let mult = 5 * 5; //25
    let modulus = 5 % 2; //1

    sumation(sum,value);
    println!("{:?}", division);
    println!("{:?}", mult);
    println!("{:?}", modulus);

}
