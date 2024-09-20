struct Item {
    id: i32,
    title: String,
    year: i32,
    type_: ItemType,
}
#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
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
}
