/*

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

*/


// solution

fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s); // function takes &mut as arguments.

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}