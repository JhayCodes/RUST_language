// Tupples are a type of recode
// Stores data anonymously, no need to name the fields unlike struct or enum
// Can be destructured

enum Access{
    Full,
}

//function returning tuple()
fn one_two_three() -> (i32,i32,i32){
    (1,2,3)
}

fn main() {
    let numbers = one_two_three();
    let (x,y,z) = one_two_three(); // this is destructuring
    println!("{:?}, {:?}", x , numbers.0); //1
    println!("{:?}, {:?}", x , numbers.1); //2
    println!("{:?}, {:?}", x , numbers.3); //3
    
    let(employee, access) = ("Jake", Accessd::Full);
}

// Recap
// Allow for anonymous data access
// Useful when destructuring
// Can be contain any number of fields
//     Use struct when more than 2 or 3 fields
