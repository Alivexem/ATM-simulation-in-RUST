use rand::Rng;
use std::io;
fn main() {
    println!("Welcome to Alive ATM machine");
    let amount_in_account = rand::thread_rng().gen_range(10_000..=200_000);
    let mut money_amount = String::new();
    let mut counter: u32 = 2;
    let mut gen_trial: u32 = 3;
    loop {
        println!("Input [insert] or [stop] to proceed");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error reading user input");
        if option.to_lowercase().trim() == "stop" {
            println!("Program ended, next time we hope you use our services!");
            break;
        } else if option.to_lowercase().trim() == "insert" {
            
            loop {
                println!("Please input the amount you wish to withdraw");
                
                money_amount.clear();
                println!("Your balance is {}", amount_in_account);
                io::stdin()
                    .read_line(&mut money_amount)
                    .expect("Error reading your withdrawal input");
                match money_amount.trim().parse::<u32>() {
                    Ok(money_amount) => {
                        if money_amount < amount_in_account {
                            println!("Withdrawal successful!");
                            println!(
                                "Your new balance is {}",
                                amount_in_account - money_amount
                            );
                            break;
                        } else if counter == 0 {
                            println!("You have exceeded your withdrawal trials!");
                            break;
                        } else {
                            println!("Insufficient Balance. You have {counter} trials left");
                            counter -= 1;
                        }
                    }
                    Err(_) => {
                        println!("Invalid input, please enter a valid number.");
                    }
                }
            }
            break;
        }else if gen_trial == 0 {
            println!("You used up all your trials, You've been logged out!");
            break;
            
        }
        else{
            println!("You inputed Gibberish, you have {} trials left", gen_trial);
            gen_trial -= 1
        }
    }
}
