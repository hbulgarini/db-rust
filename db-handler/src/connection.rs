use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use serde_bytes::Bytes;
use crate::query::Person;


pub trait DBCalls {
    fn init_db(name: &String) -> DBConnection;
    fn write_to_db(&mut self, map: HashMap<String, Person>);
}

#[derive(Debug)]
pub struct DBConnection {
    db: File,
    db_name: String,
}

impl DBCalls for DBConnection {
    
   fn init_db(name: &String) -> DBConnection {
        let db_name = format!("{}.db",name);
        println!("Initialiazing DB {}...",db_name);
        let db = File::create(db_name.clone()).unwrap();

        DBConnection {
            db,
            db_name
        }
    }
    
    fn write_to_db(&mut self,map: HashMap<String, Person>) {
        let db = Bytes::new(map);
        self.db.write_all(db).unwrap();
    }

}





