fn main() {
    //println!("Hello, world!");

    // let x;
    // {
    //     let y = 42;
    //     x = &y; //We store a reference to `y` in `x` but `y` is about to be dropped.
    // }
    // println!("x: {}", x); //`x` refers to `y` but `y has been dropped!`

    let magic1 = String::from("abracadabra!");
    let result;
    {
        let magic2 = String::from("Shazam!");
        result = longest_word(&magic1, &magic2);
    }
    println!("The longest magic word is {}", result);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }

}

struct Hightlight<'document>(&'document str);