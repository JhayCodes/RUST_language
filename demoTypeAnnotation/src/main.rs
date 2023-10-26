// Required for function signatures
// Types are usually inferred
// Can also be specified in code
//     Explicit type annotations 

//BASIC EXAMPLE
fn print_many(msg: &str, count: i32) {} //function signature.

enum Mouse{
    LeftClick,
    RightClick,
    MiddleClick,
}

fn main() {
    let num: i32 = 15;
    let a: char = 'a';
    let left_click: Mouse = Mouse::LeftClick;
    
    //GENERICS
    let numbers: Vec<i32> = vec![1,2,3];
    let letters: Vec<char> = vec!['a','b'];
    let clicks: Vec<Mouse> = vec![
        Mouse::LeftClick,
        Mouse::MiddleClick,
        Mouse::RightClick,
    ];
}

//RECAP
// Type annotations are mostly optinal within function bodies
//     Occasionally required if compiler cannot infer the type
// Can be specified when using let bindings.