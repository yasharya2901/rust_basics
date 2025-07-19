fn main() {
    let mut x = Box::new(5); // x is a Box that owns an i32

    *x = 30;
    println!("x = {}", x); // This will not compile because x is immutable
}
