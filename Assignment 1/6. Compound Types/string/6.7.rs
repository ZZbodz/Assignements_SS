/*

// Fix error with at least two solutions
fn main() {
    let s =  "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}

*/

// solution 1


// Fix error with at least two solutions
fn main() {
    let s =  "hello, world".to_string();
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}


// solution 2

// Fix error with at least two solutions
fn main() {
    let s = String::from("hello, world");
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}
