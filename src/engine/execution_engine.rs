use crate::engine::command::Command;
use crate::storage::collection::Collection;
use std::collections::VecDeque;

pub struct ExecutionEngine<C: Collection> {
    collection: C,
    transaction_stack: Vec<Command>,     
    batch_queue: VecDeque<Command>,   
    in_transaction: bool,
    in_batch: bool,
}

impl<C: Collection> ExecutionEngine<C> {
    pub fn new(collection: C) -> Self {
        Self {
            collection,
            transaction_stack: Vec::new(),
            batch_queue: VecDeque::new(),
            in_transaction: false,
            in_batch: false,
        }
    }


    pub fn execute(&mut self, command: Command) -> String {
        match command {
            Command::BeginTransaction => {
                self.in_transaction = true;
                "Transaction started".to_string()
            }
            Command::Commit => {
                while let Some(cmd) = self.transaction_stack.pop() {
                    self.execute_direct(cmd);
                }
                self.in_transaction = false;
                "Transaction committed".to_string()
            }
            Command::Rollback => {
                self.transaction_stack.clear();
                self.in_transaction = false;
                "Transaction rolled back".to_string()
            }
            Command::BatchStart => {
                self.in_batch = true;
                "Batch mode started".to_string()
            }
            Command::BatchExecute => {
                while let Some(cmd) = self.batch_queue.pop_front() {
                    self.execute_direct(cmd);
                }
                self.in_batch = false;
                "Batch executed".to_string()
            }
            cmd => {
                if self.in_transaction {
                    self.transaction_stack.push(cmd.clone());
                    "Added to transaction".to_string()
                } else if self.in_batch {
                    self.batch_queue.push_back(cmd.clone());
                    "Added to batch".to_string()
                } else {
                    self.execute_direct(cmd)
                }
            }
        }
    }


    fn execute_direct(&mut self, command: Command) -> String {
        match command {
            Command::InsertOne(_) => "Inserted 1 document".to_string(),
            Command::DeleteOne(id) => {
                if self.collection.delete_one(id) {
                    "Deleted 1 document".to_string()
                } else {
                    "No document found".to_string()
                }
            }
            Command::FindById(id) => {
                match self.collection.find_by_id(id) {
                    Some(s) => format!("Found: {:?}", s),
                    None => "No document found".to_string(),
                }
            }
            Command::FindAll => {
                let results = self.collection.find_all();
                format!("Found {} documents: {:?}", results.len(), results)
            }
            Command::Count => format!("Count: {}", self.collection.count()),
            Command::Filter(field, value) => {
                let results = self.collection.filter(&field, &value);
                format!("{} matched {} = \"{}\": {:?}", results.len(), field, value, results)
            }
            Command::Sum(field) => format!("Sum of {}: {}", field, self.collection.sum(&field)),
            Command::Average(field) => {
                format!("Average of {}: {:.2}", field, self.collection.average(&field))
            }
            Command::Import(_) => "Import received (handled in parser)".to_string(),
            _ => "Command not implemented".to_string(),
        }
    }
}