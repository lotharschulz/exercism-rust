use std::collections::{BTreeMap, HashSet};

// School struct with two fields: students_per_grade and students
pub struct School {
    students_per_grade: BTreeMap<u32, Vec<&'static str>>, // Map of grades to student names
    students: HashSet<&'static str>, // Set of all student names to ensure uniqueness
}

impl School {
    // initializes empty BTreeMap and HashSet
    pub fn new() -> School {
        School {
            students_per_grade: BTreeMap::new(), // Initialize empty BTreeMap
            students: HashSet::new(),            // Initialize empty HashSet
        }
    }

    // Add a student to a grade, ensuring no duplicates
    pub fn add(&mut self, grade: u32, student: &'static str) {
        if self.students.contains(student) {
            // Check if student is already added
            return; // If student exists, do nothing
        }
        self.students.insert(student); // Insert student into the set
        self.students_per_grade
            .entry(grade)
            .or_default()
            .push(student); // Add student to the grade
        self.students_per_grade.get_mut(&grade).unwrap().sort(); // Sort the student names
    }

    // Get a list of all grades with students
    pub fn grades(&self) -> Vec<u32> {
        self.students_per_grade.keys().cloned().collect() // Collect and return all grade keys
    }

    // Get a sorted list of students in a specific grade
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut res: Vec<_> = self
            .students_per_grade
            .get(&grade)
            .unwrap_or(&vec![])
            .iter()
            .map(|&s| String::from(s))
            .collect(); // Collect student names and convert to String
        res.sort(); // Sort the student names
        res.dedup(); // Remove duplicates
        res // Return the sorted list
    }
}
