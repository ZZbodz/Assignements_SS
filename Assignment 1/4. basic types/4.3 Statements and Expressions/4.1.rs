/*
// Make it work with two ways
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}

v is currently just an empty tuple.

so assert_eq!(v, ()); should work

*/

// solution

fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
 
    println!("Success!");
 }

// solution 2
// we can make v= x and assert it to 3
 
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }
 