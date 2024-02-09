fn main() {

    let my_number = 8;
    let my_refernce = &my_number;

    //campare value of my_number to the value of the reference to my_number
    println!("{}", my_number == *my_refernce);
}
