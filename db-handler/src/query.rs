use crate::connection::{DBConnection, DBCalls};
use std::collections::HashMap;

struct Job<'a> {
    company: &'a str,
    from: &'a str,
    to: &'a str,
    title: &'a str
}



enum TechStack {
    Javascript,
    Rust,
    Devops
}

pub struct Person<'a> {
    id: i32,
    name: String,
    lastname: String,
    jobs: Vec<Job<'a>>,
    tech_stack: Vec<TechStack>
}


impl Person {
    pub fn add_job<'a>(&mut self,id: i32, job:&'a Job){
        // get person from id   
        self.jobs.push(job);
    }
}


pub struct DBQuery {
    pub db_connection: DBConnection,
}


impl DBQuery {
    pub fn add(&mut self,registry: &String){

        let values:Vec<&str>= registry.split(";").collect();
        let name = values[0].to_string();
        let lastname = values[1].to_string();
        let jobs_provided:Vec<&str> = values[2].split(",").collect();
        let tech_stack:Vec<&str> = values[3].split(",").collect();
        
        let jobs:Vec<Job> = Vec::new();
        let jobs_iter = jobs_provided.iter();
        

        for job in jobs_iter {
            let values:Vec<&str>= job.split("#").collect();
            let job_entry = Job {
                company: values[0],
                from: values[1],
                to: values[2],
                title: values[3]
            };
            jobs.push(job_entry);
        }

        let new_person = Person {
            id: 1,
            name,
            lastname,
            jobs,
            tech_stack: Vec::new()
        };

        let mut map:HashMap<String, Person> = HashMap::new();
        map.insert(lastname, new_person);

        self.db_connection.write_to_db(map);
    }
    pub fn delete(&mut self,registry: &String){
        println!("Delete registry {}",registry);
    }
}