struct Grocery{
    id:i32,
    quantity:i32,
}

fn display_id(groceryid:&Grocery){
    println!("Id: {:?}", groceryid.id);
}

fn display_quantity(quan: &Grocery){
    println!("Quantity: {:?}", quan.quantity);   
}

fn main() {
    let drink = Grocery{
        id:7,
        quantity:12,
    };
    display_id(&drink);
    display_quantity(&drink);

}
