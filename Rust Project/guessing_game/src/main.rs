use rand::Rng;
use std::io;

fn main() {
    let fruits = [
        "apple", "banana", "cherry", "mango", "grape", "orange", "pineapple", "peach", "pear", "kiwi",
    ];

    // Generate a random index within the range of the array
    let secret_index = rand::thread_rng().gen_range(0..fruits.len());
    let secret_fruit = fruits[secret_index];

    println!("Welcome to the Guessing Game of Fruits!");
    println!("You have to guess the secret fruit from the following list of fruits:");
    println!("{:?}", fruits);
    println!("I have chosen a fruit. Can you guess which fruit it is?");
    println!("You have 5 chances to guess the fruit.");
    println!("Type your guess:");

    let mut point=5;
    loop {
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim and convert the input to lowercase
        let guess = guess.trim().to_lowercase();
        if point==0{
            println!("You have used all your guesses and u lost.");
            break;
        } else if guess == secret_fruit {
            println!("You have guessed the secret fruit correctly!");
            println!("Your point is {}",point);
            break;
        } else {
            println!("Incorrect! Try again:");
            point-=1;
        }
    }
}
