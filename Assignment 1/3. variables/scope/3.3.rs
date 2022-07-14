/*
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

both the print statements cannot access either of X or Y 
as one of them is'nt declared in their scopes

we could just remove the brackets but it would remove scopes 
so we can just declare y and x in 1st and 2nd scope

*/


//solution


fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        let y: i32 = 5;
        let x: i32 = 10;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}


