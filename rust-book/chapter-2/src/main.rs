use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    println!("gessing game");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_num}");
    println!("please enter gess number");
    let mut gess_num = String::new();
    io::stdin().read_line( &mut gess_num).expect("fail to read line");
    let guess: u32 = gess_num.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");
    match guess.cmp(&secret_num){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
