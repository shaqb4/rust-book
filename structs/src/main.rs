fn main() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someoneusername123"));

    println!("email = {}, username = {}", user1.email, user1.username);

    user1.email = String::from("anotheremail@example.com");

    println!("email = {}, username = {}", user1.email, user1.username);
    
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("email = {}, username = {}", user2.email, user2.username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}