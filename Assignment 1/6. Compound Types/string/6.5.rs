/*

// Fill the blank
fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.__("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

*/

// solution


// Fill the blank
fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats"); // replaces cats with dogs

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
