use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

pub fn create_file(path: &str, tasks: &Vec<String>) {
    if fs::metadata(path).is_ok() {
        println!("File already exists");
        return;
    }
    
    let mut file = File::create(path).unwrap();
    let now = chrono::Local::now();
    let date = now.format("%B %d, %Y").to_string();

    let mut content = format!("# {}\n\n##Tasks\n", date);
    for task in tasks {
        content.push_str(&format!("{}\n", task));
    }
    content.push_str("\n\n#Notes\n\n");

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Successfully created file: {}", path),
        Err(e) => panic!("Failed to create file: {}", e),
    }
}

pub fn create_dir(path: &str) {
    println!("Creating directory: {}", path);
    if fs::metadata(path).is_ok() {
        println!("Directory already exists");
        return;
    }
    match fs::create_dir(path) {
        Ok(_) => println!("Successfully created directory: {}", path),
        Err(e) => panic!("Failed to create directory: {}", e),
    }
}

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn get_date() -> (String, String) {
    let now = chrono::Local::now();
    let month = now.format("%B").to_string().to_lowercase();
    let day = now.format("%d").to_string().to_lowercase();
    (month, day)
}

pub fn preserve_tasks(path: &str) -> Vec<String> {
    if fs::metadata(path).is_err() {
        return Vec::new();
    }

    let mut tasks: Vec<String> = Vec::new();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let lines = contents.lines();
    for line in lines {
        if line.contains("- [ ]") {
            tasks.push(line.to_string());
        }
    }

    tasks
}
