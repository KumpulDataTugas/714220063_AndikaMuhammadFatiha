struct Counter {
    count: i32,
}

fn increment(counter: &mut Counter) {
    counter.count += 1;
}

fn main() {
    let mut c = Counter { count: 0 };
    increment(&mut c);
    increment(&mut c);
    println!("Count: {}", c.count);
}