struct Point(i32, i32);

fn print_point(p: &Point) {
    println!("Point at ({}, {})", p.0, p.1);
}

fn main() {
    let p = Point(3, 7);
    print_point(&p);
}