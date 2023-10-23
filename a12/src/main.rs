
enum BoxColor{
    Blue,
    Red,
    Brown,
}

impl BoxColor{
    fn print(&self){
        match self{
            BoxColor::Blue => println!("Blue"),
            BoxColor::Red => println!("Red"),
            BoxColor::Brown => println!("Brown"),
        }
    }
}

struct Dimensions{
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions{
    fn print(&self){
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox{
    dimensions:Dimensions,
    weight:f64,
    color: BoxColor,
}

impl ShippingBox{
    fn new(weight:f64, color:BoxColor, dimensions:Dimensions) -> Self{
        Self{
            weight,
            color,
            dimensions,
        }
    }

    fn print_box(&self){
      self.color.print();
      self.dimensions.print();
      println!("weight: {:?}", self.weight);

    }
}

fn main() {
    let small_dimensions = Dimensions{
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };


    let small_box = ShippingBox::new(5.0, BoxColor::Red, small_dimensions);
    small_box.print_box();


}
