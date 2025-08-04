use std::io;
use rand::Rng;
fn main() {
    println!("gessing game");
    let secret_num = rand::thread_rang().gen_range(1..=100);
    println!("The secret number is: {secret_num}");
    println!("please enter gess number");
    let mut gessNum = String::new();
    io::stdin().read_line( &mut gessNum).expect("fail to read line");
    println!("You guessed: {gessNum}");
}
