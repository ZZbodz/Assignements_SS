/*

fn main() {
    let s = String::from("hello, ");
    
    // Modify this line only !
    let s1 = s;

    s1.push_str("world");

    println!("Success!");
}

*/

// solution

fn main() {
    let s = String::from("hello, ");
    
    let mut s1 = s; // changing only s1.

    s1.push_str("world")
}