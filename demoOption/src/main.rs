struct Survery {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}


fn main() {
    let response = Survery{
        q1: Some(12),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1{
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no answer"),
    }
    
    match response.q2{
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no answer"),
    }
    
    match response.q3{
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no answer"),
    }

}

//refer to activity a16
