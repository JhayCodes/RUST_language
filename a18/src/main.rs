// Topic: Result

// Requirements:
// *Create a structure named 'Adult' that represents a person aged 21 or older:
// *Implement a 'new' functionality using 'derive'
#[derive(Debug)]
struct Adult {
    // *The structure must contain the person's name and age
    age: u8,
    name: String,
}
impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        //*The Ok variant should contain the initialized structure, but only if the person is aged 21 or older
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            //* The Err variant should contain a String (or &str) that explains why the structure could not be created
            Err("Age must be at least 21")
        }
    }
}

// *Implement a 'new' function for the 'Adult' stucture that returns a Result:

fn main() {
    // *Instantiate two 'Adult' structures:
    //     *One should be aged under 21
    let child = Adult::new(15, "Anita");
    //     *One should be aged 21 or over
    let adult = Adult::new(21, "Sanjay");
    // *Use 'match' to print out a message for each 'Adult':
    match child{
    //  *For the Ok variant, print any message you want
        Ok(child) => println!("{} is {}", child.name, child.age),
        Err(e) => println!("{e}"),
    }

    match adult{
        //  *For the Ok variant, print any message you want
            Ok(adult) => println!("{} is {}", adult.name, adult.age),
            Err(e) => println!("{e}"),
        }
    //     *For the Err variant, print out the error message
    println!("Hello, world!");
}
