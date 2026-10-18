use crate::models::student::Student;

#[derive(Debug, Clone)]
pub enum Command {
    InsertOne(Student),
    DeleteOne(u32),
    FindById(u32),
    FindAll,
    Filter(String, String), 
    Count,
    Sum(String),           
    Average(String),      
    Import(String),         
    BeginTransaction,
    Commit,
    Rollback,
    BatchStart,
    BatchExecute,
}