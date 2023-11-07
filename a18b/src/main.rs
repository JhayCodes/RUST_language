// Topic: Resul & the question mark operator

// Summary:
// This small program simulates unlocking a door using digital keycards
// backed by a database. many errors can occur when working with a database,
// the code managable.

// Requirements:
// *Write the body of the 'authorize' function. The steps to authorize a user are:
//     1.Connect to the database
//     2. Find the employee with the 'find_employee' database function
//     3. Get a keycard with 'get_keycard' database function
//     4. Determine if the keycard's 'access_level' is sufficient, using 'required_access_level' function implemented on 'ProtectedLocation'.
//     *Higher 'access_level' values grant more access to 'ProtectedLoctaions'.
//     1000 can access 1000 and lower, 800 can access 500 but not 1000,...
// * Run the program after writng your 'authorize' function. Expected output:
//     Ok(Allow)
//     Ok(Deny)
//     Err("Cathering doesn't have a keycard")
// * Use the question mark operator within the 'authorize' function.

#[derive(Clone, Copy, Debug)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        //In a production application, a database connection error is likely to
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee {
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee {
                name: "Brody".to_string(),
            }),
            "Catherine" => Ok(Employee {
                name: "Catherine".to_string(),
            }),
            _ => Err(String::from("employee not found")),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
        match employee.name.as_str() {
            "Anita" => Ok(KeyCard { access_level: 1000 }),
            "Brody" => Ok(KeyCard { access_level: 500 }),
            other => Err(format!("{other} doesn't have a keycard")),
        }
    }
}

#[derive(Clone, Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    // *Write the body of the 'authorize' function. The steps to authorize a user are:
    
    //     1.Connect to the database
    let db = Database::connect()?; 
    /*the ? mark does this:
    let db = match Database::connect(){
        Ok(db) = db,
        Err(e) => return Err(e)
    };*/

    
    //2. Find the employee with the 'find_employee' database function
    let employee = db.find_employee(employee_name)?;
    
    //3. Get a keycard with 'get_keycard' database function
    
    let keycard = db.get_keycard(&employee)?;
    //4. Determine if the keycard's 'access_level' is sufficient, using 'required_access_level' function implemented on 'ProtectedLocation'.
    //*Higher 'access_level' values grant more access to 'ProtectedLoctaions'.
    //1000 can access 1000 and lower, 800 can access 500 but not 1000,...
    if keycard.access_level >= location.required_access_level(){
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    //Anita is trying to access the Warehouse, which requires access level 500.
    //Her keycard has access level 1000, which should be allowed.
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);
    println!("Hello, world!");
}
