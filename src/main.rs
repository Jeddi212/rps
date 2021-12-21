use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

fn main() {
    
    const WEAPON: [u8; 3] = [1, 2, 3];
    let mut input = String::new();

    welcoming();
    read(&mut input);

    // Parse input to integer
    let input = input.to_string().trim().parse::<u8>().expect("Failed to cast input");

    // Get bot choice
    
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
