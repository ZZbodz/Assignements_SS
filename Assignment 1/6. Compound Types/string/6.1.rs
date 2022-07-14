/*

// Fix error without adding new line
fn main() {
    let s: str = "hello, world";

    println!("Success!");
}

*/

// solution


// Fix error without adding new line
fn main() {
    let s: &str = "hello, world"; // &str does the job

    println!("Success!");
}
