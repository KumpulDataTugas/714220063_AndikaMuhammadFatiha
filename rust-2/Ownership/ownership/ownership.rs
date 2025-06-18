fn main() {
    let s = String::from("test"); // s memiliki ownership atas string
    println!("{}", s); // s dapat digunakan di sini
}