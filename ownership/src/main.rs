fn main() {
    let _s = "hello";

    let mut t = String::from("Hello");
    t.push_str(", World!");
    println!("{}", t);

    //variable interactions
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    // In this case to avoid double free errors, rust takes a step to nullify s1
    // println!("{}", s1); //This should result in Error.
    println!("{}", s2);

    // deep copy example
    let y1 = String::from("Hello");
    let y2 = y1.clone();

    println!("y1 = {}, y2 = {}", y1, y2);
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);                        // s's value moves into the function and so no longer valid here.
    let x = 5;                           // x comes into scope
    makes_copy(x);      // x would move into the function but i32 is Copy, so it's okay to still use x afterwards
    //println!("x = {}, s = {}", x, s); will not work as s is moved.
    println!("x in main {}", x);

    // return values and scope.
    let s1 = gives_ownership(); // returns ownership to s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into function and some other value is returned

    println!("s1: {}", s1);
    // println!("s2: {}", s2); will not work as it's moved
    println!("s3: {}", s3);

    let (s4, len) = calculate_length(s3);
    println!("The length of s4 {} is {}", s4, len)
}// x goes out of scope then s but as s is already moved nothing special happens here.

fn gives_ownership() -> String {
    let some_string = String::from("Sunil");
    some_string
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
