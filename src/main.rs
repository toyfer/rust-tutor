#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User1 is {:?}", user1);

    let user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername567"),
        ..user1 // このように指定した場合は、定義済みのuser1と同じ値を利用する
    };

    println!("User2 is {:?}", user2);
}