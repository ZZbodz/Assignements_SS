/*

fn main() {
    for n in 1..=100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 

*/

// solution


fn main() {
    for n in 1..100 { // do not include 100
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 
