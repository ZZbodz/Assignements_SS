/*

fn main() {
    // Fix error by modifying this line
    let  s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

*/


// solution


fn main() {
    // Fix error by modifying this line
    let  mut s = String::from("hello, "); // makeing s mutable solves it.

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
