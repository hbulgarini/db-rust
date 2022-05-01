use std::fs::File;
use std::io::prelude::*;

pub fn add(mut db: File,registry: &String) {
    println!("Line to add {}",registry);
    db.write_all(registry.as_bytes()).unwrap();
}
