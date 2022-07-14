/*

// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

*/


// solution

fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s); // the function takes addreses as arguments.

    println!("Success!");
}

fn borrow_object(s: &String) {}