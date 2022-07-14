/*
// Fix the error with the use of define_x
fn main() {
    let x = define_x();
    println!("{}, world" , x);
}

fn define_x() -> String {
    "hello".to_string() // Returning the value of x
}

firstly x wasnt defined in main's  socpe so define it but we want the return value from 
define_x into x as a string so made some changes and done.

*/

//solution


fn main() {
    let x = define_x();
    println!("{}, world" , x);
}

fn define_x() -> String {
    "hello".to_string() // Returning the value of x
}



