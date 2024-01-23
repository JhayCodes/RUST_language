fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }

    // match a_number {
    //     Some(7) => println!("That's my lucky number!"),
    //     _ => {},
    // }

    //access the inner value of an option
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    // let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "candy"); //This will panic

    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    let b: Option<&str> = None;
    b.expect("fruits are healthy"); //panics with `fruits are healthy`

    //     Because these functions might panic, we don't recommend using them. Instead, consider either of the following approaches:

    // Use pattern matching and handle the None case explicitly.
    // Call similar non-panicking methods, such as unwrap_or, which returns a default value if the variant is None or the inner value if the variant is Some(value).
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}
