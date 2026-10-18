use std::collections::LinkedList;
use crate::models::student::Student;
use crate::storage::collection::Collection;

pub struct LinkedListCollection {
    data: LinkedList<Student>,
}

impl LinkedListCollection {
    pub fn new() -> Self {
        Self { data: LinkedList::new() }
    }
}

impl Collection for LinkedListCollection {
    fn insert_one(&mut self, student: Student) {
        self.data.push_back(student);
    }

    fn delete_one(&mut self, id: u32) -> bool {
        let mut cursor = self.data.cursor_front_mut();
    
        while let Some(student) = cursor.current() {
            if student._id == id {
                cursor.remove_current();
                return true;
            }
            cursor.move_next();
        }
        false
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