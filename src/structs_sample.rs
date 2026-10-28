struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[test]
fn run_structs() {
    let mut user1 = build_user("r@gmail.com".to_string(), "rizqi".to_string());

    user1.email = String::from("r2@gmail.com");

    let user2 = User {
        email: "another@gmail.com".to_string(),
        ..user1
    };

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    let Point(value1, value2, valu3) = origin;

    println!("origin: {value1}, {value2}, {valu3}");

    println!();

    println!(
        "email user1 -> {}, email user 2 -> {}",
        user1.email, user2.email
    );

    println!();

    let mut user3: User = build_user(String::from("test@gmail.com"), String::from("test"));
    println!(
        "active user 3 -> {}, sign in count -> {}",
        user3.active, user3.sign_in_count
    );
    set_inactive_user(&mut user3);
    println!(
        "active user 3 -> {}, sign in count -> {}",
        user3.active, user3.sign_in_count
    );

    println!();
    println!("======================book store ===================");
}

fn set_inactive_user(user: &mut User) -> () {
    user.active = false;
    user.sign_in_count = 0;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
