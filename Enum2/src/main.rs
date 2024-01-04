// enum WebEvent {
//     //An enum variant can be like a unit struct without fields or data types
//     WELoad,

//     //An enum variant can be like a tuple struct with data types but no named fields
//     WEKeys(String, char),

//     //An enum variant can be like a classic struct with named fields and their data types
//     WEClick { x:i64, y: i64}
// }

//this is how to use each variant on its own 

//Define a tuple struct
#[derive(Debug)]
struct KeyPress(String, char);

//Define a classic struct
#[derive(Debug)]
struct MouseClick {x: i64, y: i64 }

//Redifine the enum variants to use the data from the new structs
//Update the page Load variant to have the boolean type
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress)
}
fn main() {

    //Instatiating an enum
    let we_load = WebEvent::WELoad(true);

    //Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick {x: 100, y:250 };

    //Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    
    //Instatiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');

    //Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);

    //Print the values in the WebEvent enum variants
    //Use the {:#?} syntax to display the enum structure and data in a readable form 
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}
