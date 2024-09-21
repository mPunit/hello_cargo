#[derive(Debug)]
pub struct Price {
    price: i32,
}

impl Price {
    pub fn new(price: i32) -> Price {
        Price { price }
    }
}
