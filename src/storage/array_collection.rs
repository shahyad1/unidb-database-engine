use crate::models::student::Student;
use crate::storage::collection::Collection;

pub struct ArrayCollection {
    data: Vec<Student>,
}

impl ArrayCollection {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Collection for ArrayCollection {
    fn insert_one(&mut self, student: Student) {
        self.data.push(student);
    }

    fn delete_one(&mut self, id: u32) -> bool {
        if let Some(pos) = self.data.iter().position(|s| s._id == id) {
            self.data.remove(pos);
            true
        } else {
            false
        }
    }

    fn find_by_id(&self, id: u32) -> Option<&Student> {
        self.data.iter().find(|s| s._id == id)
    }

    fn find_all(&self) -> Vec<&Student> {
        self.data.iter().collect()
    }

    fn filter(&self, field: &str, value: &str) -> Vec<&Student> {
        self.data.iter().filter(|s| {
            match field {
                "name" => s.name == value,
                "gpa" => format!("{:.1}", s.gpa) == value || s.gpa.to_string() == value,
                "_id" => s._id.to_string() == value,
                _ => false,
            }
        }).collect()
    }

    fn count(&self) -> usize {
        self.data.len()
    }

    fn sum(&self, field: &str) -> f64 {
        if field == "gpa" {
            self.data.iter().map(|s| s.gpa).sum()
        } else {
            0.0
        }
    }

    fn average(&self, field: &str) -> f64 {
        let counter = self.count();
        if counter == 0 { 0.0 } else { self.sum(field) / counter as f64 }
    }
}