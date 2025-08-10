use std::io::{self, Write};
fn main() {
    let mut s1 = String::from("Hello");
    s1.push_str(", world");
    get_ownership(s1);
    let s2 = String::from("Hello");
    let s3 = return_ownership(s2);
    let (s3, len) = calaculate_length(s3);
    println!("s3:{}, length:{}", s3, len);
    let x = 5;
    get_copy(x);

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear();


    let mut input = String::new();

    println!("Enter your text (press Enter twice to finish):");

    loop {
        let mut line = String::new();

        io::stdout().flush().unwrap(); // Ensures prompt prints immediately

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.trim().is_empty() {
            break; // Stops on empty line (Enter twice)
        }

        input.push_str(&line);
    }

    println!("\nYou entered:\n{}", input.trim());
}

fn get_ownership(name:String){
    println!("name:{}",name);
}

fn get_copy(y:i32){
    println!("y:{}",y);
}

fn return_ownership(name:String) -> String {
    name
}

fn calaculate_length(s:String)->(String,usize) {
    let length = s.len();
    (s, length)
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

