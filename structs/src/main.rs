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
    println!("The user {} has sign in count as: {}", user2.username, user2.sign_in_count);

    let user3 = User {
        email: String::from("test3@test.com"),
        username: String::from("user3_name"),
        ..user1
    };

    println!("The third initalized user is: {}", user3.username);

    if user3.sign_in_count == user1.sign_in_count {
        println!("The count for sign in is same for user 1 and user 3!");
    } else {
        println!("The count deos not match. Check initialization again!");
    }

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    println!("The z value of black is: {}", black.2);



}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
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