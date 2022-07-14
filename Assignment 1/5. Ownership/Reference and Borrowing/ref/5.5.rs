/*

fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = __;
    
    p.push_str("world");

    println!("Success!");
}

*/

// sloution

fn main() {
    let mut s = String::from("hello, ");

    // fill the blank to make it work
    let p = &mut s; // we have use mut because we are trying to change it in the next line
    
    p.push_str("world");

    println!("Success!");
}