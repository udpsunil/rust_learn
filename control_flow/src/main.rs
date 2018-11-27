fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let other_number = 6;

    if other_number % 4 == 0 {
        println!("number is divisible by 4");
    } else if other_number % 3 == 0 {
        println!("number is divisible by 3");
    } else if other_number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // let and if statements
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {}", num);

    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let mut numeral = 3;

    while numeral != 0 {
        println!("{}!", numeral);
        numeral = numeral - 1;
    }
    println!("LIFTOFF!!!");
    assert_eq!(result, 20);

    // looping through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is :{}", a[index]);
        index += 1;
    }
    for element in a.iter() {
        println!("value is: {}", element);
    }
}
