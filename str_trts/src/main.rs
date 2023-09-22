struct Car {
    model: String,
    top_speed: i16,
}

trait Honk {
    fn honk(&self) -> &str;
}

impl Honk for Car {
    fn honk(&self) -> &str { "BEEBOP!" }
}

impl Car {
    fn new(car_model: String, speed: i16) -> Self {
        Self {
            model: car_model,
            top_speed: speed,
        }
    } 
}

fn main() {
    let car: Car = Car::new("Volvo".to_string(), 150);
    println!("{} has top speed {}", car.model, car.top_speed);
    println!("{}", car.honk());
}
