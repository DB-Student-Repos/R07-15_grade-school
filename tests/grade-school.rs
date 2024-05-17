use std::collections::BTreeMap;

#[derive(Debug)]
struct School {
    roster: BTreeMap<u32, Vec<String>>,
}

impl School {
    fn new() -> School {
        School {
            roster: BTreeMap::new(),
        }
    }

    fn add_student(&mut self, name: &str, grade: u32) {
        self.roster.entry(grade).or_insert_with(Vec::new).push(name.to_string());
        self.roster.get_mut(&grade).unwrap().sort();
    }

    fn get_students_in_grade(&self, grade: u32) -> Vec<String> {
        match self.roster.get(&grade) {
            Some(students) => students.clone(),
            None => Vec::new(),
        }
    }

    fn get_all_students(&self) -> Vec<String> {
        let mut all_students = Vec::new();
        for grade in self.roster.keys() {
            let mut students = self.roster.get(grade).unwrap().clone();
            all_students.append(&mut students);
        }
        all_students
    }
}

fn main() {
    let mut school = School::new();

    school.add_student("Jim", 2);
    school.add_student("Anna", 1);
    school.add_student("Barb", 1);
    school.add_student("Charlie", 1);
    school.add_student("Alex", 2);
    school.add_student("Peter", 2);
    school.add_student("Zoe", 2);
    school.add_student("Jim", 5);

    println!("Students in grade 2: {:?}", school.get_students_in_grade(2));
    println!("All students in the school: {:?}", school.get_all_students());
}
