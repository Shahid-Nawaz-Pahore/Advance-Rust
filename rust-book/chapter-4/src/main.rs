fn main() {
    let mut s1 = String::from("Hello");
    s1.push_str(", world");
    getOwnership(s1);
    let x = 5;
    getCopy(x);
    println!("{}", x);
}

fn getOwnership(name:String){
    println!("name:{}",name);
}

fn getCopy(y:i32){
    println!("y:{}",y);
}
