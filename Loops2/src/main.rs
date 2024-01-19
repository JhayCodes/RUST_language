fn main() {
    loop {
        // Keep printing, printing, printing...
        println!("We loop forever!");
        // On the other hand, maybe we should stop!
        break;
    }

    let mut counter = 1;
        //stop_loop is set when loop stops 
        let stop_loop = loop {
            counter *= 2;
            if counter > 100 {
                break counter;
            }
        };

        //Loop should break when counter = 128
        println!("Break the loop at counter = {}.", stop_loop);
        let mut counter_one = 0;
        while counter_one < 5 {
            println!("We loop a while...");
            counter_one = counter_one + 1;
        };

        let big_birds = ["ostrich", "peacock", "stork"];
        for bird in big_birds.iter() {
            println!("The {} is a big bird.", bird);
        }
        //Another easy way to create an iterator 
        for number in 0..5 {
            println!("{}", number * 2);
        }
        
}
