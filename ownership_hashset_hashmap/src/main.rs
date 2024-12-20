use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    
    // insert() takes ownership of the value
    set.insert(1);    // No & needed - value is moved into the set
    set.insert(2);
    
    // contains() and remove() take references because:
    // 1. They only need to look at the value, not own it
    // 2. It's more efficient than passing ownership
    if set.contains(&2) {    // Takes &i32 reference
        println!("Found 2");
    }
    
    set.remove(&1);    // Takes &i32 reference

    // For non-Copy types like String, ownership becomes more obvious
    let mut string_set: HashSet<String> = HashSet::new();
    
    let s = String::from("hello");
    string_set.insert(s);    // s is moved into the set
    // println!("{}", s);    // This would NOT compile - s was moved

    // For lookup, we still use reference
    if string_set.contains(&String::from("hello")) {
        println!("Found hello");
    }
}

// HashMap follows the same pattern
use std::collections::HashMap;

fn hashmap_example() {
    let mut map: HashMap<String, i32> = HashMap::new();
    
    // insert() takes ownership of both key and value
    let key = String::from("hello");
    map.insert(key, 42);    // key is moved
    // println!("{}", key);  // Would NOT compile
    
    // get() takes reference to key
    if let Some(value) = map.get(&String::from("hello")) {
        println!("Value: {}", value);
    }
    
    // remove() takes reference to key
    map.remove(&String::from("hello"));
}

// Understanding Copy types vs non-Copy types
fn copy_vs_move() {
    let mut set: HashSet<i32> = HashSet::new();
    
    let num = 42;
    set.insert(num);     // num is copied (i32 implements Copy)
    println!("{}", num); // This works! num was copied, not moved
    
    let mut string_set: HashSet<String> = HashSet::new();
    
    let text = String::from("hello");
    string_set.insert(text);  // text is moved (String doesn't implement Copy)
    // println!("{}", text);  // This would NOT compile
}

// Working with references in collections
fn reference_example() {
    let mut set: HashSet<&str> = HashSet::new();
    
    // &str is already a reference type
    set.insert("hello");
    
    if set.contains("hello") {  // Notice no & needed here
        println!("Found hello");
    }
}