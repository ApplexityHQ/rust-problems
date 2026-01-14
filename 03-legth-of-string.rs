// Que: Write a function get_string_length that takes a string as an input returns its length. (number of chars count)
fn main() {
    let name = String::from("my name is applexity");
    let len = get_string_length(name);
    println!("The length of the string is {}", len);
}
fn get_string_length(str: String) -> usize {
    str.chars().count()
}
