use std::fs::File;
use std::collections::HashMap;
use crate::query::Person;
use std::io::{BufWriter, Write};
use bincode::serialize_into;
use std::fs::OpenOptions;

pub trait DBCalls {
    fn init_db(name: &String) -> DBConnection;
    fn write_to_db(&mut self, map: HashMap<i32, Person>);
}

#[derive(Debug)]
pub struct DBConnection {
   pub db: File,
   pub db_name: String,
   pub new: bool
}

impl DBCalls for DBConnection {
    
   fn init_db(name: &String) -> DBConnection {
        let db_name = format!("{}.db",name);
        let db = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&db_name)
            .unwrap();
        
        let metadata = db.metadata().unwrap();
        let new = if metadata.len() == 0 { true } else { false };
        println!("DB new: {}",new);
        DBConnection {
            db,
            db_name,
            new
        }
    }
    
    fn write_to_db(&mut self,db_updated: HashMap<i32, Person>) {
        let mut f = BufWriter::new(&self.db);
        serialize_into(&mut f, &db_updated).unwrap();
    }

}





