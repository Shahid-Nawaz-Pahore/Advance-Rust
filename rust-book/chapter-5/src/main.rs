struct User {
    status:bool,
    name:String,
    email:String,
    count:i32,
}
fn main() {
    let user1 = User {
      status:true,
      name:String::from("Shahid Nawaz"),
      email:String::from("shahidnawaz.edu.pk@gmail.com"),
      count:1,
    };
    println!("User Status: {}", user1.status);
    println!("User Name: {}", user1.name);
    println!("User Email: {}", user1.email);
    println!("User Count: {}", user1.count);
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
