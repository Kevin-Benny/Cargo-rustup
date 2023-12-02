use std::io;
use rand::Rng;
use std::cmp::Ordering;

// code with run time error
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
    println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let secret_number_string = secret_number.to_string();
        match guess.cmp(&secret_number_string) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    println!("You guessed: {guess}and secret number is:{secret_number_string}");
    }
}


// Correct code
// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("Please input your guess.");
//     loop {
//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let secret_number_string = secret_number.to_string();
//         let guess = guess;
//         match guess.cmp(&secret_number_string) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//         println!("You guessed: {guess}and secret number is:{secret_number_string}");
//     }
// }
























    
// fn main() {
//     println!("Please input your guess.");
//     loop {
//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         let secret_number = rand::thread_rng().gen_range(1..=100);
//         let secret_number_string = secret_number.to_string();

//         match secret_number.cmp(&guess) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//         println!("You guessed: {guess}and secret number is:{secret_number_string}");
//     }
// }