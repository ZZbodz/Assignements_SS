/*

// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           __;
       }
       
       __
    }

    assert_eq!(n, 66);

    println!("Success!");
}

*/

// solution


// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;       // makes sure break skips until n = 66
       }
       
       break;               // breaks at n=66
    }

    assert_eq!(n, 66);

    println!("Success!");
}
