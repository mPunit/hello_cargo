pub struct Vehicle {
    make: String,
    model: String,
    year: i32,
    price: crate::vehicle::price::Price,
}

mod price;
