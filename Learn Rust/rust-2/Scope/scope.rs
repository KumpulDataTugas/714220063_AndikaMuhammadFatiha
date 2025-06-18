fn main() {
    {
        let s = String::from("In scope");
        println!("{}", s);
    }
    // println!("{}", s); // ERROR: s sudah keluar scope
}
