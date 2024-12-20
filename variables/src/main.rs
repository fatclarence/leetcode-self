fn main() {
    // let x = 5;
    // println!("The value of x is {}", x);
    // let x: &str = "6";
    // println!("The value of x is: {}", x);

    // const SUBSCRIBER_COUNT: u32 = 100_000;

    // let tup = ("Let's Get Rusty", 100_0000);
    // let (channel, sub_count) = tup;
    // println!("{} is channel, {} is subcount", channel, sub_count);

    // let sub_count = tup.1;
    // println!("{}", sub_count);

    // let byte = [0; 8];
    // let error_codes = [200, 404, 500];
    // let x = error_codes[2];

    // Returning values from a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result: {}", result);

    // Iterating over a collection
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("element: {}", element);
    }

    // Range
    for number in 1..4 {
        println!("number: {}", number);
    }
}

fn my_function(x: i32, y: i32) -> i32 {
    // do not have the semicolon if you want to return the value
    x + y
}