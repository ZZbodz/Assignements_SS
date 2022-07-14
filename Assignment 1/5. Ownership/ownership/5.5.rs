/*

// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

*/

// solution

fn main() {
    let x = (1, 2, (), "hello");
    let y = x; // Removed to_string Simply assign x to y.
    println!("{:?}, {:?}", x, y);
}