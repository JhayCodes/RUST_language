//Print 10, 20, "thirty", and 40 in a loop
//Print the total number of elements in the vector


fn main() {
    let numbers = vec![10,20,30,40];
    //numbers in for loop was being borrowed, not change of ownership
    for num in &numbers{
        //THIS
        match num {
            30 =>  println!("thirty"),
            _ => println!("{:?}", num),
        }

        //OR THIS
        // if num == 30{
        //     println!("thirty");

        // } else{
        //     println!("{:?}", num)
        // }
    }

    let total_number = numbers.len();
    println!("Total number: {:?}", total_number);
}
