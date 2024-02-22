use std::io; //standard library for input/output
use rand::Rng;
use std::cmp::Ordering; //ordering is an enum with less, greater and equal

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); //generate random number

    loop {
    
     //print the secret number
    println!("Please input the number"); //general print statements

    let mut guess = String::new(); //create a variable to store the guess, mutable means it can change later, bound to a new string

    io::stdin().read_line(&mut guess).expect("Failed to read line"); //input the number from stdin

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    }; //shadowing 
    //trim removes white spaces from the string
    //parse converts the string to a different type

    println!("you guessed: {guess}"); //print the guessed number

    //cmp method compares two values, returning the value of ordering enum.
    //match expression decides what to do on the basis of the comparison
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("guess is too small"),
        Ordering::Greater => println!("guess is too big"),
        Ordering::Equal => {
            println!("guess is right!");
            println!("secret number was: {secret_number}");
            break;
        }
    }
}
}
