/*

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: __ = __;
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

*/

// solution


fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4]; //  its from 1 to 4(not included)
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
