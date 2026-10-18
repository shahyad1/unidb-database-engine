use crate::models::student::Student;

pub trait Collection {
    fn insert_one(&mut self, student: Student);

    fn delete_one(&mut self, id: u32) -> bool;

    fn find_by_id(&self, id: u32) -> Option<&Student>;

    fn find_all(&self) -> Vec<&Student>;

    fn filter(&self, field: &str, value: &str) -> Vec<&Student>;

    fn count(&self) -> usize;

    fn sum(&self, field: &str) -> f64;

    fn average(&self, field: &str) -> f64;
}