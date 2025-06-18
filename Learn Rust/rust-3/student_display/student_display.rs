struct Student {
    name: String,
    grade: char,
}

fn display(s: &Student) {
    println!("{} got grade {}", s.name, s.grade);
}

fn main() {
    let s = Student {
        name: String::from("Rektw"),
        grade: 'A',
    };
    display(&s);
}
