// enum is a type that can represent one item at a time 
//     Each item is called a variant
// enum is not limited to just plain variants 
//     Each variant can optionally contain additional data

enum Mouse{
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32,i32),
}

enum PromoDiscount{
    NewUser,
    Holiday(String),
}

enum Discount{
    Percentage(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

fn main() {
    println!("Hello, world!");
}


// //RECAP
// enum variants can optionally contain data  
//     The data can be another enum
// Can mix plain identifiers and data-containing variants within the same enum
// More than one piece of data can be associated with a variant.