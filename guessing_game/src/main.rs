use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!"); //name game

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate number
    //println!("The secret number is: {secret_number}"); //print number
    loop {
        println!("Please input your guess.");// acquire input 
        let mut guess = String::new(); //create guess variable
        // put the typed input into the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //process input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}"); //say the guess
        // process guess and determine if you should retry or if you win
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }    
}
