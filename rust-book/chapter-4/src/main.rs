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


    let arr = [10, 20, 30, 40, 50];

    let slice = &arr[1..4]; // Elements at index 1, 2, 3
    println!("Slice: {:?}", slice); // [20, 30, 40]
    let text = String::from("Hello, Rust!");

    let hello = &text[0..5]; // "Hello"
    let rust = &text[7..11]; // "Rust"

    println!("{} - {}", hello, rust);


    let arr = [1, 2, 3, 4];
    let slice = &arr[..]; // entire array
    println!("{:?}", slice); // [1, 2, 3, 4]
    let mut numbers = [1, 2, 3, 4, 5];

    let slice = &mut numbers[1..4]; // mutable borrow of part of the array
    for num in slice.iter_mut() {
        *num *= 10; // multiply each element by 10
    }

    println!("{:?}", numbers); // [1, 20, 30, 40, 5]

    
    let nums = vec![5, 10, 15, 20];
    println!("Sum: {}", sum(&nums[1..3])); // sum of [10, 15]
}

fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
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

