use std::io;
fn main() {
    println!("Hello, world!");
    println!("you are at branch Checkpoint!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    
    //panic!("Problem reading:{:?}",Err::<T, ()>(()));
    //code to print error from reading line as well as the result
    //io::stdin().read_line(&mut guess).expect(Err(()));
    
    

    println!("You guessed: {guess}");
}
