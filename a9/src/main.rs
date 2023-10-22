//Data management using tuples

//function that returns a tuple
fn coord() -> (i32, i32) {
    (8, 7)
}

fn main() {
    // let _x = coord().0;
    // let y = coord().1;
    let (_x, y) = coord();

    if y > 5 {
        println!("y-coordinate is greater than 5");
    } else if y < 5 {
        println!("y-coordinate is less than 5");
    } else {
        println!("y-coordinate is equal to 5");
    }
}
