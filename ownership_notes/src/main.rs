// Moving:
// Transfers ownership
// No memory allocation (just changes who owns the data)
// Original variable can't be used anymore
// Very efficient (zero cost)

// Cloning:

// Creates a deep copy of the data
// Requires new memory allocation
// Original can still be used
// More expensive operation

// Borrowing:

// Just creates a reference
// No memory allocation
// Original can still be used (with some restrictions)
// Very efficient (just a pointer)

// Understanding for loops and borrowing:
// for &num in nums.iter() {
//     // nums.iter() produces references (&i32) to the elements
//     // &num in the pattern destructures the reference
//     // so num is just the i32 value (which is on the stack since it's a primitive)
// }


fn main() {
    println!("Hello, world!");

    let mut vec1: Vec<String> = Vec::new();
    // String::from allocates on the heap which will be slower
    // heap passes back a pointer to where the string is stored
    vec1.push(String::from("Hello"));
    vec1.push(String::from("World"));
    println!("{:?}", vec1);

    let s2: String = String::from("Poop");
    let mut s3: String = takes_and_gives_back(&s2);
    println!("{}", s2);

    // a variable can only have one mutable reference to it in a scope
    modifies_string(&mut s3);
    println!("{}", s3);

    let nums_array: [i32; 5] = [1, 2, 3, 4, 5];
    let two_numbers = two_numbers(&nums_array);
    println!("{:?}", two_numbers);
}

// Passing in references as parameters is known as borrowing
// Borrowing is a concept in Rust that allows you to pass references to data without taking ownership of it.
// References are immutable by default, but you can make them mutable by adding a mut keyword.
fn takes_and_gives_back(a_string: &String) -> String {
    a_string.clone()
}

// If we want to modify the original value, we can use a mutable reference.
fn modifies_string(a_string: &mut String) {
    a_string.push_str(" World");
}

// // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// // Slices are defined by a pointer to the first element and a length.
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// Slices also work for arrays of integers
fn two_numbers(s: &[i32]) -> &[i32] {
    &s[..2]
}