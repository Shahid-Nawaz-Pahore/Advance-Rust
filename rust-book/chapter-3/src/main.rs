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
    println!("The difference is: {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("The product is: {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient is: {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("The truncated value is: {}", truncated);
    // remainder
    let remainder = 43 % 5;
    println!("The remainder is: {}", remainder);
    let x = 80;
    if x >= 80 {
        println!("pass");
    } else {
        println!("fail");
    }


    // if-else if
    let y = 6;
    if y%4 == 0 {
        println!("{} is divisible by 4", y);
    } else if y%3 == 0 {
        println!("{} is divisible by 3", x);
    } else if y%2 == 0 {
        println!("{} is divisible by 2", y);
    } else {
        println!("{} is not divisible by 4 or 3", y);
    }


    // if in let statement
    let condition = true;
    let number = if condition {10} else {20};
    println!("The value of number is: {}", number);

}
