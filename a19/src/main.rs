// Topic: HashMap
use std::collections::HashMap;
// Requirements:
// *Print the name and number of items in stock for a furniture store 
// *If the number of items is 0, print "out of stock" instead of 0
// *The store has:
//     *5 chairs
//     *3 Beds
//     *2 Tables
//     *0 Couches
// * Print the total number of items in stock

// Notes:
// * Use a HashMap for the furniture store stock
// #[derive(Debug)]
// struct Furniture {
//     name: String,
//     number: i32,
// }
fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chairs", 5);
    stock.insert("Beds", 3);
    stock.insert("Tables", 2);
    stock.insert("Couches", 0);
    let mut total_stock = 0;

    for (furniture, number) in stock.iter() {
        total_stock = total_stock + number;
        let stock_count = if number == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", number)
        };
        println!("items={:?}, stock={:?}", furniture, stock_count);
    }

    println!("total stock={:?}", total_stock);

    // for (furniture, number) in stock.iter() {
    //     if number > &0 {
    //         println!("furniture = {:?}, number = {:?}", furniture, number);
    //     } else {
    //         println!("Total number of {:?} is out of stock", furniture);
    //     }
    // }

}
