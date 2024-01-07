

fn main() {

    //Declare array, initialize all values, compiler infers Length = 7
    let days = ["Sunday", "Monday","Tuesday","Wednesday","Thursday","Friday","Saturday"];

    //Declare array, initialize all values to 0, length = 5
    let bytes = [0,5];

    // Get the second day of the week
    let second = days[1];

    // Get the seventh day of the week using the wrong index - should be 6
    let seventh = days[7];
    println!("Hello, world!");
}
