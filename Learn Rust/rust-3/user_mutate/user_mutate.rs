struct User {
    name: String,
    age: u8,
}

fn birthday(user: &mut User) {
    user.age += 1;
}

fn main() {
    let mut user = User { name: String::from("rektw"), age: 20 };
    birthday(&mut user);
    println!("{} is now {} years old", user.name, user.age);
}
