/*
fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.__ {
        println!("The {}th element is {}",i+1,v);
    }
}

*/

// solution

fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {   // enumerate() does the job
        println!("The {}th element is {}",i+1,v);
    }
}
