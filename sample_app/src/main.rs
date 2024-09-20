struct Student {
    name: String,
    grade: Option<u32>,
}

fn check(name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if *name == student.name {
            return student.grade;
        }
    }
    None
}

fn main() {
    let student_db = vec![
        Student {
            name: "Bob".to_string(),
            grade: Some(56),
        },
        Student {
            name: "Alice".to_string(),
            grade: Some(87),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        },
    ];

    let name = "Charlie".to_string();

    let student_grade = check(&name, &student_db);

    if let Some(grade) = student_grade {
        println!("Grade is {grade}");
    }
}
