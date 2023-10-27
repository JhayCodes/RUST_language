enum Discount{
    Percent(i32), 
    Flat(i32),
}

struct Ticket{
    event: string,
    price: i32,
}

fn main() {
    let n = 3;
    match n{
        3 => println!("three"),
        //do this instead
        other => println!("number: {:?}", other);
    }
    //create enum discount variant with extra data.
    let flat = Discount::Flat(2);

    //match enum
    match flat{
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (), //ignore the percentage variant
    }

    //match struct
    let concert = Ticket{
        event: "concert".to_owned(),
        price: 50.0,
    }
    match concert{
        Ticket{ price:50, event} => println("event @ 50 = {:?}", event),//match to price 50 in the ticket struct and print out event.
        Ticket{ price, ..} => println("price = {:?}", price),//match to the ticket struct and fetch only the price.
    }
}
