// pass reference of string s1 to another function using & operator.
// & operator allows to refer some value without taking its ownership.
fn main() {
    let s1 = String::from("Hello World!");
    let len = calculate_length(&s1);    //s1 is passed by references hence ownership remains with main block.
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("Hello");      // we can have only one mutable reference to a piece of data
    //let r1 = &mut s;
    //let r2 = &mut s;    // This fails with cannot borrow `s` as mutable more than once at a time
    // Benefit of this is rust prevents data races at compile time.
    // instead use this
    {
        let r1 = &mut s;
        println!("Printing using r1 {}", r1);
    }
    {
        let r2 = &mut s;
        println!("Printing using r2 {}", r2);
    }

    // mutable and immutable references
    {
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        //let r3 = &mut s; // big problem
        println!("Printing using ref {}", r1);
        println!("Printing using ref {}", r2);
        //println!("Printing using ref {}", r3);
    }
    change(&mut s);
    println!("The string after change is '{}'.", s);

    // Dangling references
    // let reference_nothing = dangle();
    let reference_something = no_dangle();
    println!("Printing no_dangle {}", reference_something);
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn calculate_length(some_str: &String) -> usize {
    some_str.len()
} // some_str goes out of scope here but since it's only pointing to some other value, pointer goes out of scope.

fn change(some_str: &mut String) {
    some_str.push_str(", World!!");
}