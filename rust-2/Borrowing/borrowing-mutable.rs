fn modify(s: &mut String) {
    s.push_str("Test");
}

fn main() {
    let mut s = String::from("one");
    modify(&mut s); // pinjam mutable
    println!("{}", s);
}