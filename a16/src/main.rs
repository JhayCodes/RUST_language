// Topic: Option

// Requirement:
// *Print out the details of a student's locker assignment
// *Lockers use numbers and are optional for students

struct Student{
    name: String,
    locker: Option<i32>,
}

fn main() {
    let mark = Student{
        name: "Mark".to_owned(),
        locker: Some(21),
    };
    let jane = Student{
        name: "Jane".to_owned(),
        locker: None,
    };
    println!("Student: {:?}", mark.name);
    match mark.locker{
        Some(ans) => println!("Locker: {:?}", ans),
        None => println!("No locker number"),
    }

    println!("Student: {:?}", jane.name);
    match jane.locker{
        Some(ans) => println!("Locker: {:?}", ans),
        None => println!("No locker number"),
   
    }

}
