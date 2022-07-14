/*
// Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
}

*/

// solution

fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// add return type and return string

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}