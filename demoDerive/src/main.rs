//The can be automatically enumerated for your structs and enums
//There's a clone and copy derives. informs the compiler to automaticaly make copies. 
//Ownership isn't transfered buh a copy is made instead.
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Clone, Copy)] //All fields must also derive the functionality, so add the derive to the Position.
struct Employee{
    position: Position,
    work_hours: i64
}

fn print_employee(emp: Employee){
    println!("{:?}", emp); //if not for derive:clone, copy. functionality, employee should be borrowed or references with &.
}

fn main() {
    let me = Employee{
        position: Position::Worker,
        work_hours: 40
    };

    //Instead of this:
    match me.position{
        Position::Manager => println!("manager"),
        Position::Supervisor => println!("supervisor"),
        Position::Worker => println!("worker")
    }

    // //You can add a derive functionality on struct/enum and it's variants and do this:
    // println!("{:?}", me);

    print_employee(me);
    print_employee(me);
    

}

//Clone and Copy should only be applied to sructs that are small in size
//else epensive copies would be made without you knowing 
