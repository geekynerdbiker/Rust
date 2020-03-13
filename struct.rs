fn main() {
    let mut st1 = Student {
        name: String::from("Jacob"),
        major: String::from("ITCAE"),
        grade: 2,
        status: false,
    };

    println!("name: {}, major: {}, grade: {}, status: {}", st1.name, st1.major, st1.grade, st1.status);
    st1.grade = 3;
    println!("name: {}, major: {}, grade: {}, status: {}", st1.name, st1.major, st1.grade, st1.status);

    let st2 = add_student(String::from("Alice"), String::from("Social"));
    println!("name: {}, major: {}, grade: {}, status: {}", st2.name, st2.major, st2.grade, st2.status);

    let st3 = Student {
        name: String::from("Bob"),
        major: String::from("Mechanic"),
        ..st1
    };
    println!("[FRIEND] name: {}, major: {}, grade: {}, status: {}", st3.name, st3.major, st3.grade, st3.status);
}

struct Student {
    name: String,
    major: String,
    grade: isize,
    status: bool,
}

fn add_student(name: String, major: String) -> Student{
    Student {
        name,
        major,
        grade: 1,
        status: true
    }
}
