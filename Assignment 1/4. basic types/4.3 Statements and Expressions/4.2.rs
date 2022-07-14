
/*

fn main() {
   let v = (let x = 3);

   assert!(v == 3);

   println!("Success!");
}

let x= 3 can be used in {}.
*/

// solution


fn main() {
    let v = {let x = 3;
        x
    };
 
    assert!(v == 3);
 
    println!("Success!");
 }
 