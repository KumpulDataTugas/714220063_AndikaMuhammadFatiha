fn print_length(s: &String) {
    println!("Length is: {}", s.len());
}

fn main() {
    let s = String::from("Borrow");
    print_length(&s); // pinjam tanpa mengambil ownership
    println!("{}", s); // s masih bisa dipakai
}
