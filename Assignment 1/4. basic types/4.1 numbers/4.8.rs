/*

fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
}

*/


// solution 

fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32); // 1

    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001); //2
    
    println!("Success!");
}





