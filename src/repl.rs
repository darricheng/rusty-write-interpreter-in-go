use std::io;

pub fn start() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");
        println!("Input: {}", input)
    }
}
