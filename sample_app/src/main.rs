use sample_app::{price, return_bus, year, Bus, Car};

fn main() {
    let car = Car::new("Elantra".to_string(), 2023, 2000);

    let bus = Bus::new(50, 2021, 25000);

    println!("Price of the car is {}", car.calculate_price());
    println!("Price of the bus is {}", bus.calculate_price());

    year(car);
    return_bus();
}
