use std::io::Seek;
use std::{fs::File, io::Write};
use std::collections::BTreeMap;
use codec::Encode;

use crate::query::{Person,Id};
use std::fs::OpenOptions;

pub trait DBCalls {
    fn init_db(name: &String) -> DBConnection;
    fn write_to_db(&mut self, map: BTreeMap<Id, Person>);
}

#[derive(Debug)]
pub struct DBConnection {
   pub db_file: File,
   pub db_name: String,
   pub new: bool
}

impl DBCalls for DBConnection {
   fn init_db(name: &String) -> DBConnection {
        println!("Calling init...");
        let db_name = format!("{}.db",name);

         let db_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&db_name)
            .unwrap();

        let metadata = db_file.metadata().unwrap();
        let new = if metadata.len() == 0 { true } else { false };
        println!("DB new: {}",new);

        DBConnection {
            db_file,
            db_name,
            new
        }
    }

    fn write_to_db(&mut self,db_updated: BTreeMap<Id, Person>) {
        let encoded = db_updated.encode();
        println!("write_to_db ${:?}",db_updated);
        self.db_file.seek(std::io::SeekFrom::Start(0)).expect("can't rewind the cursor");
        self.db_file.write(&encoded).unwrap();
    }
}





