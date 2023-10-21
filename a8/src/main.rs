// Notes:
// *Use an enum to create different flavors of drinks
enum DrinkFlavor{
    Cola,
    Lime,
}

// *Use a struct to store drink flavor and fluid ounce information
struct Drink{
    flavor: DrinkFlavor,
    fluid:f64,
}

// *Use a function to print out the drink flavor and ounces
 // *Use a match expression to print the drink flavor

fn print_drink(drink : Drink){
    match drink.flavor{
        DrinkFlavor::Cola => println!("flavor: Cola"),
        DrinkFlavor::Lime => println!("flavor: Lime"),

    }
    println!("Fluid: {:?}", drink.fluid);
}


fn main() {
    let coke = Drink {
        flavor: DrinkFlavor::Cola,
        fluid: 1.8,
    };
     print_drink(coke);
    println!();
    let fanta = Drink {
        flavor: DrinkFlavor::Lime,
        fluid: 1.5,
    };
    print_drink(fanta);

   
    
}
