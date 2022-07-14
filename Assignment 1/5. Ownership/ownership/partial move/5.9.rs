/*

fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (__, __) = __;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

*/


// solution


fn main() {
    let t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (s1 , s2) = t.clone(); // first two are clearly s1 and s2 third blank has to be a clone of t as we are using it again in the print
 
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }
 