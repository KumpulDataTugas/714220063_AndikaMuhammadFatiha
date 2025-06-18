struct Car {
    brand: String,
    year: u16,
}

fn new_car(brand: &str, year: u16) -> Car {
    Car {
        brand: brand.to_string(),
        year,
    }
}

fn main() {
    let car = new_car("Toyota", 2024);
    println!("Car: {}, Year: {}", car.brand, car.year);
}