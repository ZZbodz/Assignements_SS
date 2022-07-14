/*
// Fill the blanks in the code to make it compile
fn main() {
    let __ =  1;
    __ += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

The blank would be x as x was used in assert, and we would have to use 'mut' 
to be able to change x.

*/

//solution


fn main() {
    let mut x =  1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}


