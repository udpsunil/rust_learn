fn main() {
    println!("Hello, world!");
    another_function(5, 6);
    // Error
    // let x = (let y = 6);
    let _y = {
        let x = 3;
        x + 1   // last statement gets assigned (no need of ;) This is called an expression.
        // Line ending with ; is a statement.
    };
    let x = five();
    println!("The value of x is: {}", x);

    let y = plus_one(x);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {} and y is {}!", x, y);
}

fn five() -> i32 { 5 }

fn plus_one(x: i32) -> i32 { x + 1 }