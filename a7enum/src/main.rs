enum Colors{
    Red, 
    Yellow, 
    Green, 
    Black
}

fn print_color(my_color : Colors){
    match my_color{
        Colors::Red => println!("Red"),
        Colors::Yellow => println!("Yellow"),
        Colors::Green => println!("Green"),
        Colors::Black => println!("Black"),
    }
}

fn main() {
    print_color(Colors::Red);
}
