struct Point<T> {
    x: T,
    y: T,
}

struct SecondPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let boolean = Point {x: true, y: false};
    let integer = Point {x: 1, y: 9};
    let float = Point {x: 1.7, y: 4.3};
    let string = Point {x: "hight", y: "low"};

    //Though `T` can assume any concreate type, x and y must be of the same type.
    //The code on line 19 will not work. You can comment it out to avoid errors.
    let wont_work = Point {x: 25, y: true};

    //Multiple generic data type can be used to solve the issue above.
    let integer_and_boolean = SecondPoint{x: 5, y: false};
    let float_and_string = SecondPoint{x: 1.0 y: "hey"};
    let integer_and_float = SecondPoint{x:5, y:4.0};
    let both_integer = SecondPoint {x: 10, y: 30};
    let both_boolean = SecondPoint{x: true, y: false};
    println!("Hello, world!");
}
