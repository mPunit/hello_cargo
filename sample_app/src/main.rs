use sample_app::{Car, Price, Vehicle};

struct Item {
    id: i32,
    title: String,
    year: i32,
    type_: ItemType,
}
#[derive(Debug)]
enum ItemType {
    Book,
    _Magazine,
}

impl Item {
    fn display_item_info(&self) {
        println!(
            "The id is {}, title is {}, year is {}, type is {:#?}",
            self.id, self.title, self.year, self.type_
        );
    }
}

fn main() {
    let book1 = Item {
        id: 12,
        title: "harry".to_string(),
        year: 2005,
        type_: ItemType::Book,
    };

    book1.display_item_info();
    let price: Price = Price::new(38000);
    let vehicle = Vehicle::new("Hyundai".to_string(), "Elantra".to_string(), 2023, price);
    let car = Car::new(vehicle);

    println!("The car details are: {:?}", car);
}
