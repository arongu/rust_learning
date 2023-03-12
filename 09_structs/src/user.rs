struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // init shorthand
        username, // init shorthand syntax
        active: true,
        sign_in_count: 1,
    }
}

pub(crate) fn one() {
    let mut user1 = User {
        email: String::from("aron@test.com"),
        username: String::from("aron"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("arongu");

    let user2 = build_user(String::from("user2@test.com"), String::from("usr2"));

    let user3: User = User {
        email: String::from("user3@test.com"),
        ..user2 // using destructuring
    };
}
