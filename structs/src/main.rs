

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_acount: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_acount: 0,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from(""),
        email: String::from(""),
        sign_in_acount: 0,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("user2"),
        email: String::from("email2"),
        ..user1
    };

    
}
