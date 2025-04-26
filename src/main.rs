mod options;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "start" => println!("Started"),
        "stop"  => println!("Stopped"),
        _       => println!("No such command!"), 
    }
                   
    
}
