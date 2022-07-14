/*

// Fill the blank
fn main() {
    let mut s = __;
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

*/

// solution


// Fill the blank
fn main() {
    let mut s = String:new(); // its new because we are asserting s to hello, world!
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
