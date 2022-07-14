/*

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.into_bytes();
    s
}

*/

// solution

fn main() {
    let s = give_ownership();
    println!("{}", s);
}


fn give_ownership() -> String {
    // basic way to return string
    let s = String::from("hello, world");
    s
}