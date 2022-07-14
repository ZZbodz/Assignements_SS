/*

// Fill in the blanks
fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } __ n > 0 {
        println!("{} is positive", n);
    } __ {
        println!("{} is zero", n);
    }
} 

*/

// solution


// Fill in the blanks
fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {                           // else if
        println!("{} is positive", n);
    } else  {                                   // else
        println!("{} is zero", n);
    }
} 
