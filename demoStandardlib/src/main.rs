fn main() {
    let numbers = vec![1,2,3];
    match numbers.is_empty() {
        ture => println!("no numbers"),
        false => println!("has numbers"),
    }
}

//go to terminal, type rustup doc to open the standard library docmentation
// click on extensive API documentation
// Locate vector mudule
// click vec in the vector structure
// then look for the method