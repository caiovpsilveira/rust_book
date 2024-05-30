#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let user = User {
        name: String::from("Caio"),
        age: 22,
        email: String::from("caiovpsilveira@gmail.com"),
    };

    println!("Original user info: {:?}", user);

    let user2 = copy_user(&user);

    println!("Copied user info: {:?}", user2);
}

fn copy_user(user: &User) -> User {
    User {
        name: user.name.clone(),
        email: user.email.clone(),
        age: user.age,
    }
}
