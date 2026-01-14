// Que: Check if a number is even.

fn main () {
    let ans = is_even(20);
    println!("{}", ans);
}

// i  means signed (i32), and u means unsigned (u32)
fn is_even (num: i64) -> bool {
    if num%2 == 0 {
        return true;
    } else {
        return false;
    }
}
