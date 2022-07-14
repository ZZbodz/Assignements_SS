/*


fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}

*/

//solution

fn main() {

    //  cloning

    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);


    // reference

    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
    

    // slicing
    
    let x = String::from("hello, world");
    let y = &x[..];
    println!("{},{}",x,y);

}