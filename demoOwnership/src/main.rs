// Managing Memory
// *Programs must track memory
//     If they fail to do so, a "leak" occurs
// *Rust utilizes an "ownership" model to manage memory
//     The "owner" of memory is responsible for cleaning up the memory
// *Memory can either be "moved" or "borrowed"
// *Owner in RUST refers to a function.

enum Light{
    Bright, 
    Dull,
}

fn display_light(light: &Light){
    match light{
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull); //there's going to be an error because the variable dull was 
    //moved to another function, display_light and after the function was executed, it deleted
    // the variable as a result of memory management.

    //What to do to fix the result, is to borrow the variable from the main function by adding & to the variable
    //declared in display_light function and anywhere dull is used
    display_light(&dull);

}

// //RECAP
// *Memory must be managed in some way to prevent leaks
// *Rust uses "ownership" to accomplish memory management 
//     The "owner" of data must clean up the memeory
//     This occurs automatically at the end of the scope
// *Default behavior is to "move" memory to a new owner
//     Use an ampersand (&) to allow code to "borrow" memory
