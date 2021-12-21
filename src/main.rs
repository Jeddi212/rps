use rand::Rng;
use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    input.clear();
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read input");
}

fn main() {
    
    let mut rng = rand::thread_rng();
    let mut input = String::new(); 

    loop {
        welcoming();
        read(&mut input);
        
        // Parse input to integerYouYouYou
        let input = input.to_string().trim().parse::<u8>().expect("Failed to cast input");

        match input {
            1 | 2 | 3 => (),
            _ => {
                let mut exit = String::new();
                print!("Wish to exit? (yes/no) : ");
                read(&mut exit);
                if wish_exit(&exit) {
                    println!("\nThank you\n");
                    break;
                }
                println!();
                continue;
            },
        }

        // Get bot choice {0 [Rock], 1 [Paper], 2 [Scissor]}
        let bot: u8 = rng.gen_range(1..4);

        // Check Winner
        match (&input, &bot) {
            (1, 1) | (2, 2) | (3, 3) => print_result("Draw", &input, &bot),
            (2, 1) | (3, 2) | (1, 3) => print_result("Win", &input, &bot),
            (1, 2) | (2, 3) | (3, 1) => print_result("Lose", &input, &bot),
            _ => println!("Unknown Condition"),
        };
    }

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

fn wish_exit(exit: &str) -> bool {
    if exit.to_lowercase().trim() == "yes" {
        return true;
    }
    false
}

fn print_result(result: &str, human: &u8, bot: &u8) {
    println!("
You choose : {}
Bot choose : {}
Result     : {}
-----------------\n", human, bot, result);
}