/*


// Fill in the blank
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    __ {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
*/

// solution


// Fill in the blank
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {                            // using if let
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
