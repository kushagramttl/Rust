fn main() {
    let mut user1 = User {
        email: String::from("test@test.com"),
        username: String::from("user_name"),
        sign_in_count: 2,
        active: true
    };

    println!("user has been made with email id: {}", user1.email);

    user1.active = false;

    println!("User activity updated to: {}", user1.active);

    let user2 = build_user(String::from("test2@test.com"), String::from("user2_name"));
    println!("The second initalized user is: {}", user2.username);
    println!("The user {} has sign in count as: {}", user2.username, user2.sign_in_count)

}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 32
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}