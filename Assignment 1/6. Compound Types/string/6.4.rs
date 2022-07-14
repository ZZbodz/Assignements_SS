/*

// Fix all errors without adding newline
fn main() {
    let  s =  String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}

*/

// solution

fn main() {
    let mut s = String::from("hello"); // if we make s mutable we could remove to_string() also
     s.push(',');
     s.push_str(" world");
     s += "!";
 
     println!("{}", s)
 }