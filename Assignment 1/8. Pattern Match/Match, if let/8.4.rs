/*

fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(__)
    }

    println!("Success!");
} 

*/

// solution


fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))  // Returns whether the given expression matches any of the given patterns.
                                                                    // basically if's                                                                
    }

    println!("Success!");
} 
