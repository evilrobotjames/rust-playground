
use std::io;

fn main() {

    let mut nth = String::new();
    println!("Enter fibonacci sequence index:");

    io::stdin().read_line(&mut nth).expect("failed to readline");
    
    let nth: u16 = nth.trim()
        .parse()
        .expect("not a positive integer");

    if nth == 0 {
        eprintln!("not a positive integer");
        std::process::exit(1);
    }

    println!("target: {nth}");

    let mut iterations_left = nth - 1;

    let mut current = 0;
    let mut next = 1;
    
    while iterations_left > 0 {
        let tmp = current + next;
        current = next;
        next = tmp;
        iterations_left = iterations_left - 1;
    }

    println!("{current}");
    
}
