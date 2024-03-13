fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("someuser@somedomain.com"),
        sign_in_count: 0
    };
    if user1.active {
        println!("{} ({}) : {} sign-ins",
            user1.username, user1.email, user1.sign_in_count);
    }

    let user2 = build_user("anotheruser", "anotheruser@somedomain.com");
    if user2.active {
        println!("{} ({}) : {} sign-ins",
            user2.username, user2.email, user2.sign_in_count);
    }
}

fn build_user(username: &str, email: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}