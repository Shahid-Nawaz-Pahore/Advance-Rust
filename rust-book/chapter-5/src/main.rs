struct User {
    status:bool,
    name:String,
    email:String,
    count:i32,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let subject = AlwaysEqual;

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
        status: user1.status,
        name: user1.name,
        email: String::from("another@example.com"),
        count: user1.count,
    };

}
fn build_user(email: String, name: String) -> User {
    User {
        status: true,
        name,
        email,
        count: 1,
    }
}
