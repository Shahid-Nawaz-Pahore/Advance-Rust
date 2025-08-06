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
