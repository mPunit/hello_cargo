mod vehicle {
    pub struct Vehicle {
        make: String,
        model: String,
        year: i32,
        price: crate::vehicle::price::Price,
    }

    mod price {

        pub struct Price {
            price: i32,
        }
    }
}

mod car {
    use crate::vehicle::Vehicle;

    struct Car {
        details: Vehicle,
    }
}
