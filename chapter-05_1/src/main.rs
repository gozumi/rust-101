struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(user1.username, user1.email);

    let user3 = User {
        email: String::from("someone_else@here.com"),
        username: String::from("Someone Else"),
        ..user1
    };

    let black = Color(1, 2, 4);
    let origin = Point(34, 54, 23);
    let origin2: (i32, i32, i32) = (1, 11, 33);

    let (x, y, z) = origin2;

    let user4 = User {
        email: "anotherPerson@there.com",
        username: "anotherperson",
        active: true,
        sign_in_count: 1,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
