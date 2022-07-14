/*

fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, y);

    println!("Success!");
}

*/

// solution


fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y); // y hold the address not the value in x.

    println!("Success!");
}
