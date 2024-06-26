use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!"); // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  //小さすぎ！
            Ordering::Greater => println!("Too big!"), //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                break;
            } //やったね！
        }
    }
}
