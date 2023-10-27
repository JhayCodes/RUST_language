//Print out a list of tickets and their information fo an event
// Tickets can be Backstage, Vip, and Standard
// Backstage and Vip tickets include the ticket holder's name
// All tickets include the price
enum Ticket{
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(40.0, "Billy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(60.0, "Amy".to_owned()),
    ];

    for ticket in tickets{
        match ticket{

            Ticket::Backstage(price, holder) => println!("Backstage ticket Holder: {:?}, price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Standard price: {:?}", price),
            Ticket::Vip(price, holder) => println!("Vip ticket holder: {:?}, price: {:?}", holder, price),
        }
    }
}
