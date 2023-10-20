fn main() {
    let mut i:i32 = 1;
    loop {
        println!("{:?}", i);

        if i == 4 {
            break;
        }
        i = i + 1;
    }
}
