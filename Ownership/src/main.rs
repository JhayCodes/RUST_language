fn main() {
    fn caller() {
        let s = String::from("Hello, world!");
        process(s.clone());
        //process(s.clone());
        process(s);
        
    }
}

fn process(input: String){}
//Every call to ".clone" makes a full copy of the data. This can make your code slower.
//This method includes memory allocations and other expensive operations, "borrowing" values avoids these costs.