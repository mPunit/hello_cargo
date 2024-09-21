pub use price::Price;
#[derive(Debug)]
pub struct Vehicle {
    make: String,
    model: String,
    year: i32,
    price: Price,
}

impl Vehicle {
    pub fn new(make: String, model: String, year: i32, price: Price) -> Vehicle {
        Vehicle {
            make,
            model,
            year,
            price,
        }
    }
}

mod price;
