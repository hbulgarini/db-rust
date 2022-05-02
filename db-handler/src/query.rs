use crate::connection::{DBConnection, DBCalls};
use core::num::flt2dec::Decoded;
use std::{collections::BTreeMap, io::Read, io::Write};
use std::io::BufWriter;
use std::fmt::Debug;
use codec::{Encode, Decode};



#[derive(Encode, Decode, Debug)]
struct Job {
    company:String,
    from:String,
    to:String,
    title:String
}

#[derive(Encode, Decode, Debug)]
enum TechStack {
    Javascript,
    Rust,
    Devops
}

#[derive(Encode, Decode, Debug, Eq, PartialOrd, Ord, PartialEq)]
pub struct Id {
    id: i32
}

#[derive(Encode, Decode, Debug)]
pub struct Person {
    id: Id,
    name: String,
    lastname: String,
    jobs: Vec<Job>,
    tech_stack: Vec<TechStack>
}



pub struct DBQuery {
    pub db_connection: DBConnection,
}


impl DBQuery {
    fn open(&mut self) -> (BTreeMap<Id, Person>, Id) {
        
        let db: BTreeMap<Id, Person> = if self.db_connection.new == true {  
            BTreeMap::new()
        } else {
/*             let db_file = &
            let file = Decode::decode(input)
            bincode::deserialize(file).unwrap() */
            let mut buf:Vec<u8> = vec![]; 
            let file = self.db_connection.db_file.read_to_end(&mut buf).unwrap();
            let db = BTreeMap::decode(&mut buf).unwrap();
            db
        };
        let current_id = Id { id: 1};
        println!("DB Before updating ${:?}: ", db);
        (db, current_id)
    }

    pub fn add(&mut self,registry: &String){
        let values:Vec<&str>= registry.split(";").collect();
        let name = values[0].to_string();
        let lastname = values[1].to_string();
        let jobs_provided:Vec<&str> = values[2].split(",").collect();
        //let tech_stack:Vec<&str> = values[3].split(",").collect();
        
        let mut jobs:Vec<Job> = Vec::new();
        let jobs_iter = jobs_provided.iter();
        

        for job in jobs_iter {
            let values:Vec<&str>= job.split("#").collect();
            let job_entry = Job {
                company: values[0].to_string(),
                from: values[1].to_string(),
                to: values[2].to_string(),
                title: values[3].to_string()
            };
            jobs.push(job_entry);
        }

        let (mut db_updated, last_id) = self.open();

        let new_person = Person {
            id: Id {id: last_id.id +1 }, 
            name,
            lastname,
            jobs,
            tech_stack: Vec::new()
        };

        println!("new_person ${:?}: ",new_person);

        db_updated.insert(Id {id: last_id.id +1 },new_person);

        println!("DB After updating ${:?}: ",db_updated);
        self.db_connection.write_to_db(db_updated); 
    }

    pub fn show(&self){
        let (db_updated, last_id) = self.open();
        println!("${:?}: ",db_updated);
    }

    pub fn delete(&mut self,registry: &String){
        println!("Delete registry {}",registry);
    }
}