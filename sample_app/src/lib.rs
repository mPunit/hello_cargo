pub use vehicle::{price, Bus, Car};

mod vehicle {
    pub struct Car {
        model: String,
        year: i32,
        kms: i32,
    }

    pub struct Bus {
        seats: i32,
        year: i32,
        kms: i32,
    }

    impl Car {
        pub fn new(model: String, year: i32, kms: i32) -> Car {
            Car { model, year, kms }
        }
    }

    impl Bus {
        pub fn new(seats: i32, year: i32, kms: i32) -> Bus {
            Bus { seats, year, kms }
        }
    }

    pub trait price {
        fn calculate_price(&self) -> i32;
    }

    impl price for Car {
        fn calculate_price(&self) -> i32 {
            20000
        }
    }

    impl price for Bus {
        fn calculate_price(&self) -> i32 {
            30000
        }
    }
}
