/*

// Implement struct Point to make it work.


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}

*/

// solution


// Implement struct Point to make it work.
struct Point<T, U>{
    x : T,
    y : U,                      // defining a generic struct named Point with x and y as struct variables
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}
