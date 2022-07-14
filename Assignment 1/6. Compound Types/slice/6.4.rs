/*

fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[__];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

*/


// solution


fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[0..=1]; // from 0 to 1( 1 included )

    assert_eq!(slice1, slice2);

    println!("Success!");
}
