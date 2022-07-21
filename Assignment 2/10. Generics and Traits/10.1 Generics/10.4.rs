/*

// Modify this struct to make the code work
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}

*/

// solution


// Modify this struct to make the code work
struct Point<T, U> {
    x: T,                           // use two generic types instead of just one
    y: U,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}
