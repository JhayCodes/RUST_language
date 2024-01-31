fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn change(text: &mut String) {
    text.push_str(", world"); //We try to add a "!" to the end of our message
}

fn main() {
    let mut greeting = String::from("Hello");
    // print_greeting(&greeting);//`print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    // print_greeting(&greeting); //Since `greeting` didn't move into `print_greeting` we can use it again.

    change(&mut greeting);
    print_greeting(&greeting);
}


// Your code must implement either of the following definitions, but not both at the same time:

// One or more immutable references (&T)
// Exactly one mutable reference (&mut T)

//Eg: The following codes will have errors

// fn main() {
//     let mut value = String::from("hello");

//     let ref1 = &mut value;
//     let ref2 = &mut value;

//     println!("{}, {}", ref1, ref2);
// }

// fn main() {
//     let mut value = String::from("hello");

//     let ref1 = &value;
//     let ref2 = &mut value;

//     println!("{}, {}", ref1, ref2);
// }
