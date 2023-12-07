use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
            println!("Input a guess: ");
            
            let mut guess= String::new();
            // ^^^ created a mutable variable bound to a new, empty instance of a String
            io::stdin() // call stdin() from the io module
            .read_line(&mut guess)
            // ^^^ take std input and store it in the variable by reference
            .expect("Failed to read line");
        // ^^^ handle the scenario where input couldn't be read
        let guess: u32 = guess.trim().parse().expect("Please only input positive integers");
        //println!("You guessed the following: {guess}");
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Equal => { println!("You win"); break;},
            Ordering::Greater => println!("Too big")
        }
    }
}
