/*

// Make it work
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

by switching && with || would make the t true and assert correct.

*/


// Make it work
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}
