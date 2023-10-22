// Expressions
// Rust is an expression-based language
// Most things are evaluated and return some value
// Expression values coalesce to a single point.
//     Can be used for nesting logic
enum Access{
    Admin, 
    Manager,
    User,
    Guest,
}

fn main() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 {
        true
    } else {
        false
    };

    //let is_lt_5 = my_num < 5;

    //it can also contain match expression
    let message = match my_num{
        1 => "hello",
        _ => "goodbye"
    };

    //secret file: admins only
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("Can access? {:?}", can_access_file)
}
//Recap
// Expressions allow nested logic
// if and match expressions can be nested
//     Best to not use more than two or three levels
