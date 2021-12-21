use std::io::{stdin, stdout, Write};

pub fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

pub fn welcoming() {
    println!("Rock Paper Scissor Game!!!");
    print!("Please choose your weapon
1. Rock
2. Paper
3. Scissor
4. Exit
-> ");
}

pub fn wish_exit(exit: &str) -> bool {
    if exit.to_lowercase().trim() == "yes" {
        return true;
    }
    false
}

pub fn print_result(result: &str, human: &u8, bot: &u8) {
    println!("
You choose : {}
Bot choose : {}
Result     : {}
-----------------\n", human, bot, result);
}