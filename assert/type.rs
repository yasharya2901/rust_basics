use std::mem::size_of_val;

fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 1);


    let c2: char = 'b';
    assert_eq!(size_of_val(&c2), 1);

    println!("Success!")
}