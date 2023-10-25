//Print out the name and fovorite colors of people aged 10 and under

struct Persons{
    age:i32,
    name: String,
    color: String,
}

fn print_name(name:&str){
    println!("name: {:?}", name);
}

fn print_color(color:&str){
    println!("color: {:?}", color);
}


fn main() {

    let people = vec![
        Persons{
            age:8,
            name: "Joshua".to_owned(),
            color: "Blue".to_owned(),   
        },
        Persons{
            age:11,
            name: String::from("Emma"),
            color: "Black".to_owned(),
        },
        Persons{
            age:2,
            name: "Joy".to_owned(),
            color: String::from("Pink"),
        }
    ];

    for person in people{
        if person.age <= 10{
            print_name(&person.name);
            println!("age: {:?}", person.age);
            print_color(&person.color);
            println!();
        }
    }
}
