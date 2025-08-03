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


}
