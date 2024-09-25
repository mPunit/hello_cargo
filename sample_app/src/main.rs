// -------------------------------------------
// 			Iterator
// -------------------------------------------

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
#[derive(Debug)]
struct Employee {
    name: String,
    salary: u16,
}

#[derive(Debug)]
struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };

    let emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30_000,
    };

    let emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());

    for employee in emp_db {
        println!("{employee}");
    }
}
