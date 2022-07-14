/*

// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

*/

// solution


// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;  // we cant have two mutable borrows.
    let r2 = &s;  // so only two immputable borrows are possible.

    println!("{}, {}", r1, r2);

    println!("Success!");
}
