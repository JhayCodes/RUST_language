//Struct is a type that contains multiple pieces of data.
// Each piece of data is called a "field". 
// Makes working with data easier because similar data can be grouped together
//Example
struct ShippingBox{
    depth: i32,
    width: i32,
    height: f64,
}


fn main() {
    let my_box = ShippingBox{
        depth: 3,
        width: 2, 
        height: 5,
    }
    //individual fields can be accessed using the dot operator
    let tall = my_box.height;
    println!("The box is {:?} units tall", tall);
}

// Recap
// Structs deal with multiple pieces of data
// All fields must be present to create a Structs
// Fields can be accessed using the a dot(.)
//Refer to "Activity 8"