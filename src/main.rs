use rand::Rng;
use rps::*;

fn main() {
    
    let mut rng = rand::thread_rng();
    let mut input = String::new(); 

    loop {
        welcoming();
        read(&mut input);
        
        // Check is user just enter the input || is not a number
        if &input == "\n" || !input.clone().contains("1234567890") {
            println!();
            continue;
        }

        // Parse input to integer
        let input = input.to_string().trim().parse::<u8>().expect("Failed to cast input");
        
        // Matching if user input is in weapon range
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

        // Get bot choice {1 [Rock], 2 [Paper], 3 [Scissor]}
        let bot: u8 = rng.gen_range(1..=3);

        // Check Winner
        match (input, bot) {
            (1, 1) | (2, 2) | (3, 3) => print_result("Draw", input, bot),
            (2, 1) | (3, 2) | (1, 3) => print_result("Win", input, bot),
            (1, 2) | (2, 3) | (3, 1) => print_result("Lose", input, bot),
            _ => println!("Unknown Condition"),
        };
    }

}
