use crate::connection::{DBConnection, DBCalls};
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use bincode::deserialize_from;

#[derive(Deserialize, Serialize, Debug)]
struct Job {
    company:String,
    from:String,
    to:String,
    title:String
}
#[derive(Deserialize, Serialize, Debug)]
enum TechStack {
    Javascript,
    Rust,
    Devops
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    id: i32,
    name: String,
    lastname: String,
    jobs: Vec<Job>,
    tech_stack: Vec<TechStack>
}

pub struct DBQuery {
    pub db_connection: DBConnection,
}


impl DBQuery {
    fn open(&self) -> (HashMap<i32, Person>, i32) {
        let db: HashMap<i32, Person> = if self.db_connection.new == true {
            HashMap::new()
        } else {
            let file = &self.db_connection.db;
            deserialize_from(file).unwrap()
        };
        let current_id = db.len().try_into().unwrap();
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
            id: last_id +1, 
            name,
            lastname,
            jobs,
            tech_stack: Vec::new()
        };

        println!("new_person ${:?}: ",new_person);

        println!("DB Before updating ${:?}: ",db_updated);
        db_updated.insert(last_id+1,new_person);
        println!("DB After updating ${:?}: ",db_updated);
        self.db_connection.write_to_db(db_updated);
    }

    pub fn show(&self){
        let (db_updated, last_id) = self.open();
        println!("${:?}: ",db_updated);
        println!("Records :${}",last_id);
    }

    pub fn delete(&mut self,registry: &String){
        println!("Delete registry {}",registry);
    }
}