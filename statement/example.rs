fn main() {
    let x: i32 = 2;
    let y = {
        let squared = x * x;
        let cubed = squared * x;
        squared + cubed;
    };

    println!("The result is: {}", y);
}