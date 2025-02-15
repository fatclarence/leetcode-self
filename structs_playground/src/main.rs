struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("John Doe"),
        email: String::from("john.doe@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // username is moved
    };

    // will fail to compile because user1.username is moved
    // println!("user1: {:?}", user1.username);
}