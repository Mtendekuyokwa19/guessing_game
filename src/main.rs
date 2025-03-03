use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        println!("guess a number");

        println!("Please input your guess");
        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("the secret_number is {secret_number}");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read_line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
