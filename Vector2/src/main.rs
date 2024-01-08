fn main() {
    //Declare vector, initialize with three values
    let three_num = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_num);

    //Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    //Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    //Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Push integer value, but vector expects String (&str) type value
    // fruit.push(1);

    //Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    //Add 5 to the value at index 1, which is 5 + 1 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);

    // Access vector with out-of-bounds index
    let beyond = index_vec[10];
    println!("{}", beyond);
}
