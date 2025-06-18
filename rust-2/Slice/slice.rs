fn main() {
    let s = String::from("Hello, Rustaceans");
    let hello = &s[0..5]; // slice string
    println!("Slice: {}", hello);
}