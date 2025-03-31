use std::io::{self, Write};
fn main() {

    let mut input_line = String::new();

    print! ("Enter a number, I'll tell you whether it is \x1b[1meven \x1b[0mor \x1b[1modd\x1b[0m! ");
    io::stdout().flush().unwrap();
    
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    
    
    let x: i32 = input_line.trim().parse().expect("Input not an integer");

    if (x % 2) == 0 {
        println!("{} is \x1b[1modd\x1b[0m!", x);      
    } else {
        println!("{} is \x1b[1meven\x1b[0m!", x);
    }    
}
