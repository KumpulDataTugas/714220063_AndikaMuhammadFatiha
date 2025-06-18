struct Temperature {
    celsius: f32,
}

impl Temperature {
    fn to_fahrenheit(&self) -> f32 {
        (self.celsius * 1.8) + 32.0
    }
}

fn main() {
    let t = Temperature { celsius: 37.0 };
    println!("In Fahrenheit: {}", t.to_fahrenheit());
}