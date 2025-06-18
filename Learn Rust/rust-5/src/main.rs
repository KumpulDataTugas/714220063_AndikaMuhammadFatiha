mod my_utils;

fn main() {
    my_utils::greet("World");
    
    let result = my_utils::add(10, 20);
    println!("The result is: {}", result);
}