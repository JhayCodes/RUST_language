// *Multiple pieces of data
//     Must be the same type(struct, enum, string, numbers)
// *Used for lists of information
// *Can add, remove, and tranverse the entries

//Example

let my_numbers = vec!{1,2,3};

let mut my_numbers = Vec::new();
my_numbers.push(1);//adds 1 to vector
my_numbers.push(2);//adds 2 to vector
my_numbers.push(3);//adds 3 to vector
my_numbers.pop();//removes the last item in the vector
my_numbers.len();//displays length of the vector

let two = my_numbers[1]; //assing item in index 1 to variable two. this is called slice.

fn main() {
    let my_numbers = vec!{1,2,3};

    //loop through vector and print out numbers.
    for num in my_numbers{
        println!("{:?}", num);
    }
}

// RECAP
// Vectors contain multiple pieces of similar data
// Data can be added or removed
// The vec! macro can be used to make vector
// Use for.. in to itereate through items of a vector

//Refer to activity a13
