#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(username: String, email: String, active: bool, sign_in_count: u64) -> User {
    User {
        username,
        email,
        active,
        sign_in_count,
    }
}
fn main() {
    let user = build_user("KhaiTc".to_string(), "duy@gmail.com".to_string(), true, 20);
    let user2 = User {
        email: String::from("ABC@gmail.com"),
        ..user
    };
    println!("Infomation {:?}", user2);
}
