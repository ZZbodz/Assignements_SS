/*
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(__)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(__) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}

*/

// solution

struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(self: &Self)  {                         // &self can be written as self: &Self
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(self: &mut Self) {                  // &mut self can be written as self: &mut Self
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}
