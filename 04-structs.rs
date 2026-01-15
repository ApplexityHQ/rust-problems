struct User {
    first_name: String,
    last_name: String,
    age: i32,
}
fn main() {
    let user = User {
        first_name: String::from("Applexity"),
        last_name: String::from("Ox"),
        age: 20,
    };

    println!(" First Name : {} Last Name: {} Age: {}" , user.first_name, user.last_name, user.age);
}
