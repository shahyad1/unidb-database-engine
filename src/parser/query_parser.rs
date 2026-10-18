use crate::engine::command::Command;
use crate::models::student::Student;
use serde_json::{Value, from_str};

pub fn parse_query(input: &str) -> Result<Command, String> {
    let trimmed = input.trim().trim_end_matches(';').trim();

    if trimmed.starts_with("db.students.insertOne(") {
        parse_insert_one(trimmed)
    } else if trimmed == "db.students.findAll()" {
        Ok(Command::FindAll)
    } else if trimmed.starts_with("db.students.findById(") {
        parse_find_by_id(trimmed)
    } else if trimmed.starts_with("db.students.deleteOne(") {
        parse_delete_one(trimmed)
    } else if trimmed == "db.students.count()" {
        Ok(Command::Count)
    } else if trimmed.starts_with("db.students.filter(") {
        parse_filter(trimmed)
    } else if trimmed.starts_with("db.students.sum(") {
        parse_sum(trimmed)
    } else if trimmed.starts_with("db.students.average(") {
        parse_average(trimmed)
    } else if trimmed == "beginTransaction()" {
        Ok(Command::BeginTransaction)
    } else if trimmed == "commit()" {
        Ok(Command::Commit)
    } else if trimmed == "rollback()" {
        Ok(Command::Rollback)
    } else if trimmed == "batch.start()" {
        Ok(Command::BatchStart)
    } else if trimmed == "batch.execute()" {
        Ok(Command::BatchExecute)
    } else {
        Err(format!("Unknown command: {}", input))
    }
}

fn extract_json(input: &str) -> Result<String, String> {
    let start = "insertOne(".len();
    let json_str = &input[start..input.len() - 1];
    Ok(json_str.trim().to_string())
}

fn parse_insert_one(input: &str) -> Result<Command, String> {
    let json_str = extract_json(input)?;
    let value: Value = from_str(&json_str).map_err(|e| format!("Invalid JSON: {}", e))?;

    let _id = value["_id"].as_u64().ok_or("Missing _id")?;
    let name = value["name"].as_str().ok_or("Missing name")?.to_string();
    let gpa = value["gpa"].as_f64().ok_or("Missing gpa")?;

    Ok(Command::InsertOne(Student::new(_id as u32, name, gpa)))
}

fn parse_find_by_id(input: &str) -> Result<Command, String> {
    let start = "findById(".len();
    let id_str = &input[start..input.len() - 1];
    let id = id_str.trim().parse::<u32>().map_err(|_| "Invalid ID")?;
    Ok(Command::FindById(id))
}

fn parse_delete_one(input: &str) -> Result<Command, String> {
    let start = "deleteOne(".len();
    let id_str = &input[start..input.len() - 1];
    let id = id_str.trim().parse::<u32>().map_err(|_| "Invalid ID")?;
    Ok(Command::DeleteOne(id))
}

fn parse_filter(input: &str) -> Result<Command, String> {
    let start = "filter(".len();
    let args = &input[start..input.len() - 1];
    let parts: Vec<&str> = args.split(',').map(|s| s.trim().trim_matches('"')).collect();
    if parts.len() != 2 {
        return Err("Filter needs field and value".to_string());
    }
    Ok(Command::Filter(parts[0].to_string(), parts[1].to_string()))
}

fn parse_sum(input: &str) -> Result<Command, String> {
    let start = "sum(".len();
    let field = &input[start..input.len() - 1];
    Ok(Command::Sum(field.trim_matches('"').to_string()))
}

fn parse_average(input: &str) -> Result<Command, String> {
    let start = "average(".len();
    let field = &input[start..input.len() - 1];
    Ok(Command::Average(field.trim_matches('"').to_string()))
}