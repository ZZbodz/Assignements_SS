/*

// Use two approaches to fix the error and without adding a new line
fn main() {
    let s =  "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}

*/

// solution 1


// Use two approaches to fix the error and without adding a new line
fn main() {
    let s =  "hello, world".to_string();
    let s1: &str = &s; // matching types.

    println!("Success!");
}

// solution 2


// Use two approaches to fix the error and without adding a new line
fn main() {
    let s =  "hello, world"; // matching types
    let s1: &str = s;

    println!("Success!");
}

