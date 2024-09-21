use crate::vehicle::Vehicle;
#[derive(Debug)]
pub struct Car {
    details: Vehicle,
}

impl Car {
    pub fn new(details: Vehicle) -> Car {
        Car { details }
    }

    pub fn get_details(&self) -> &Vehicle {
        &self.details
    }
}
