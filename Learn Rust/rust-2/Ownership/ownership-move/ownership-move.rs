fn main() {
    let s1 = String::from("Rust test");
    let s2 = s1; // ownership pindah ke s2
    // println!("{}", s1); // ERROR: s1 sudah tidak memiliki ownership
    println!("{}", s2);
}