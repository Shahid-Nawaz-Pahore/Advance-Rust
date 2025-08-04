use std::io;
fn main() {
    println!("gessing game");
    println!("please enter gess number");
    let mut gessNum = String::new();
    io::stdin().read_line( &mut gessNum).expect("fail to read line");
    println!("You guessed: {gessNum}");
}
