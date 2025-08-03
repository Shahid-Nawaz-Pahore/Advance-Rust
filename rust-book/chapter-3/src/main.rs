fn main() {
    //println!("Hello, world!");
    let x=5;
    let y=8;
    println!("the value of x is {} and the value of y is {} and sum is {}",x,y, x+y);
    let mut x = 20;
    x  += x;
    println!("{0} + {0} = {1}", x,x+x);

let spaces = "   ";
    let spaces = spaces.len();
    println!("the length of spaces is {}", spaces);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 is moved here
    println!("{}", s3);
    // addition
    let sum = 5 + 10;
    println!("The sum is: {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

}
