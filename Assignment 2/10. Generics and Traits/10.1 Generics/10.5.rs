/*

// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}

*/

// solution


// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val<T> {
    val: T,             // added genric type T
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        // because I added T type for the struct i have to use the genric type in implementaion also
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}
