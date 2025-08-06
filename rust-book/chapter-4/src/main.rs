fn main() {
    let mut s1 = String::from("Hello");
    s1.push_str(", world");
    getOwnership(s1);
    let s2 = String::from("Hello");
    let s3 = returnOwnership(s2);
    println!("s3:{}", s3);
    let x = 5;
    getCopy(x);
}

fn getOwnership(name:String){
    println!("name:{}",name);
}

fn getCopy(y:i32){
    println!("y:{}",y);
}

fn returnOwnership(name:String) -> String {
    name
}
