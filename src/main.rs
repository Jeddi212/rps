use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

fn main() {
    
    let mut input = String::new();

    welcoming();
    read(&mut input);

    println!("{}", input);
}

fn welcoming() {
    println!("Rock Paper Scissor Game!!!");
    print!("Please choose your weapon
1. Rock
2. Paper
3. Scissor
4. Exit
-> ");
}