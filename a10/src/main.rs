

fn print_result(result:bool){
    match result{
        true => println!("Number is greater than 100"),
        _ => println!("Number is less than 100")
    }
}
fn main() {
    let num = 770;
    let is_num_big = num > 100;
    
    print_result(is_num_big);
}
