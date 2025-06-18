struct Pair {
    a: i32,
    b: i32,
}

fn max(pair: &Pair) -> i32 {
    if pair.a > pair.b { pair.a } else { pair.b }
}

fn main() {
    let p = Pair { a: 5, b: 8 };
    println!("Max is {}", max(&p));
}