//use codec::Encode;
use std::io::{Seek, Write};
use std::{fs::File};

use crate::query::{DB};
use std::fs::OpenOptions;

pub trait DBCalls<'a> {
    fn init_db(name: &String) -> DBConnection;
    fn write_to_db(&mut self, map: &DB);
}

#[derive(Debug)]
pub struct DBConnection {
    pub db_file: File,
    pub db_name: String,
    pub new: bool,
}

impl<'a> DBCalls<'a> for DBConnection {
    fn init_db(name: &String) -> DBConnection {
        println!("Calling init...");
        let db_name = format!("{}.db", name);

        let db_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&db_name).expect(&format!("Can't open the file: {}", &db_name));


        let metadata = db_file.metadata().unwrap();
        let new = if metadata.len() == 0 { true } else { false };
        println!("DB new: {}", new);

        DBConnection {
            db_file,
            db_name,
            new,
        }
    }

    // fn open(&mut self) -> (DB<'static>, Id) {
    //     if self.new == true {
    //         return (DB::new(), Id { id: 0 });
    //     } else {
    //         let mut content = String::new();
    //         self.db_file.read_to_string(&mut content).expect("can't read file!");
    //         // let mut buf: Vec<u8> = vec![];

    //         // {
    //         //     self.db_file.read_to_end(&mut buf).unwrap();
    //         // }
    //         // let input = &buf[..];

    //         let db: DB = serde_json::from_str(&content).expect("can't decode json!");

    //         let current_id = db.len();
    //         return (
    //             db,
    //             Id {
    //                 id: current_id as u32,
    //             },
    //         );
    //     };
    // }

    fn write_to_db(&mut self, db_updated: &DB) {
        //let encoded = db_updated.encode();
        println!("{:?}", &db_updated);
        let db_json = serde_json::to_string(db_updated).expect("can't encode db as json!");
        self.db_file
            .seek(std::io::SeekFrom::Start(0))
            .expect("can't rewind the cursor");
        self.db_file.write(db_json.as_bytes()).expect("can't write file!");
    }
}
