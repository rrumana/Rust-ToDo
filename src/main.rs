use std::collections::HashMap;
use clap::Parser;
use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::Display;

/// Simple CLI to-do list
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The action to perform; add, complete, remove, or print
    #[arg(short, long)]
    action: String,

    /// The description of the item
    #[arg(
        short, 
        long, 
        required_if_eq_any([
            ("action","add"),
            ("action","complete"),
            ("action","remove")
        ]))]
    description: Option<String>,
}

struct Todo {
    map: HashMap<String, String>,
}

impl Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in &self.map {
            write!(f, "{}: {}\n", key, value)?;
        }
        Ok(())
    }
}

impl Todo {
    fn new() -> Result<Todo> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => Err(anyhow!("Error reading the database from the file: {}", e)),
        }
    }

    fn insert(&mut self, key: String) -> Result<()> {
        self.map.insert(key, String::from("To Do"));
        self.save()?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: String) -> Result<()> {
        match self.map.get_mut(&key) {
            None => return Err(anyhow!("Could not find key: {}", key)),
            Some(_) => self.save()?,
        }
        Ok(())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        match self.map.remove(&key) {
            None => return Err(anyhow!("Could not find key: {}", key)),
            Some(_) => self.save()?,
        }
        Ok(())
    }
}

fn handle_commands(
    action: String, 
    description: String, 
    mut list: Todo,
) -> Result<()> {
    match action.as_str() {
        "add" => list.insert(description)?,
        "complete" => list.complete(description)?,
        "remove" => list.remove(description)?,
        "print" => { 
            println!("{}", list); 
            return Ok(()) 
        },
        _ => panic!("Invalid command"),
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let action = args.action;
    let description = match args.description {
        Some(desc) => desc,
        None => String::new(),
    };

    let list = match Todo::new() {
        Ok(list) => list,
        Err(e) => panic!("Database initialization failed: {}", e),
    };

    match handle_commands(action, description, list) {
        Err(err) => println!("An error occurred executing command: {}", err),
        _ => (),
    }
}
