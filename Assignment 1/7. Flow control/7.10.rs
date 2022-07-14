/*

// Fill in the blank
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            __;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

*/

// solution


// Fill in the blank
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter*2; // returning 20
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}
