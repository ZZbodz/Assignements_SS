/*

// Fix the error with at least two solutions
fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

*/

// solution 1


// Fix the error with at least two solutions
fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(&s) // &s as an argument.
}

fn greetings(s: &str) {
    println!("{}",s)
}


// solution 2


// Fix the error with at least two solutions
fn main() {
    let s: Box<&str> =  "hello, world".into();
    greetings(*s) // make s & str and use it as argument
}

fn greetings(s: &str) {
    println!("{}",s)
}
