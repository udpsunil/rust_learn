fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is {}", guess);

    // integer overflow example
    let mut x: u8 = 0;
    x = x + 1;
    while x > 0 {
        x = x + 1;
        println!("Value of x is : {}", x);
    }
    println!("X is overflown and it's value now is {}", x)
}
