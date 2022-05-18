//use crate::connection::{DBCalls, DBConnection};
//use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::{collections::BTreeMap};

use crate::connection::{DBConnection, DBCalls};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Job<'a> {
    company: &'a str,
    from: &'a str,
    to: &'a str,
    title: &'a str,
}

impl Display for Job<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "     Job: Company: {}, From: {}, To: {}, Title: {}",
            self.company, self.from, self.to, self.title
        )
    }
}

pub type DB<'a> = BTreeMap<u32, Person<'a>>;

#[derive(Debug, Clone)]
enum TechStack {
    Javascript,
    Rust,
    Devops,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person<'a> {
    name: &'a str,
    lastname: &'a str,
    jobs: Vec<Job<'a>>,
    //tech_stack: Vec<TechStack>,
}

impl Display for Person<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Name: {}, Last Name: {}", self.name, self.lastname)?;
        for job in &self.jobs {
            write!(f, "\n{}", job)?;
        }
        write!(f, "")
    }
}

pub struct DBQuery<'a> {
    pub db: DB<'a>
}

// <<<<<<< Updated upstream
// impl DBQuery {
//     fn open(&mut self) -> (DB, Id) {
//         if self.db_connection.new {
//             return (DB::new(), Id { id: 0 });
//         } else {
//             let mut buf: Vec<u8> = vec![];
//             self.db_connection.db_file.read_to_end(&mut buf).unwrap();
//             let mut input = &buf[..];

//             let db = DB::decode(&mut input).unwrap();

//             let current_id = db.len();
//             return (
//                 db,
//                 Id {
//                     id: current_id as u32,
//                 },
//             );
//         };
//     }

//     fn generate_person(registry: &str) -> Result<Person,()> {
//         let values: Vec<&str> = registry.split(";").collect();
//         if let [name, lastname, job_provided] = &values[..] {
//             let jobs = job_provided.split("#").collect::<Vec<&str>>().iter().map(|job| {
//                 let values: Vec<&str> = job.split(",").collect();
//                 Job {
//                     company: values.get(0).unwrap_or(&"").to_string(),
//                     from: values.get(1).unwrap_or(&"").to_string(),
//                     to: values.get(2).unwrap_or(&"").to_string(),
//                     title: values.get(3).unwrap_or(&"").to_string(),
//                 }
//             }).collect::<Vec<Job>>();

//             let person = Person {
//                 name: name.to_string(),
//                 lastname: lastname.to_string(),
//                 jobs,
//                 tech_stack: Vec::new(),
//             };
// =======
impl<'a> DBQuery<'a> {
    fn generate_person(registry: &'a str) -> Result<Person<'a>,()> {
        let values: Vec<&str> = registry.split(";").collect();
        if let [name, lastname, jobs_provided] = &values[..] {

            let mut jobs: Vec<Job> = Vec::new();

            for job in jobs_provided.split(",").collect::<Vec<&str>>().iter() {
                println!("{}",job);
                let values: Vec<&str> = job.split("#").collect();
                let job_entry = Job {
                    company: values[0],
                    from: values[1],
                    to: values[2],
                    title: values[3],
                };
                jobs.push(job_entry);
            }

            let person = Person {
                name,
                lastname,
                jobs,
                //tech_stack: Vec::new(),
            };
            Ok(person)
        } else {
            eprintln!("Invalid registry!");
            Err(())
        }
    }


    pub fn add(&mut self, db_connection: &mut DBConnection,  registry: &'a str) {

        let next_id = self.db.len() as u32 + 1;
        if let Ok(new_person) = DBQuery::generate_person(&registry) {
            self.db.insert(next_id, new_person);
            db_connection.write_to_db(&self.db);
            self.show(next_id);
        }
    }

    pub fn show_all(&self) {

        for (key, value) in self.db.iter() {
            println!("Id {} {}", key, value);
        }
    }


    pub fn show(&self, id: u32) {
        println!("Row Id: {} Data: {:?}", id, self.db.get(&id));
    }

    pub fn update(&mut self, mut db_connection: DBConnection, id: u32, update: &'a str) {

        let last_id: u32 = self.db.len() as u32;
        if id >= last_id + 1 {
            panic!("No existing record with that Id");
        }
        println!("Updating row with id: {}", id);
        if let Ok(updated_row) = DBQuery::generate_person(&update) {
            self.db.insert(id, updated_row.clone());
            db_connection.write_to_db(&self.db);
            self.show( id);

        }

    }

    pub fn delete(&mut self, registry: &str) {
        println!("Delete registry {}", registry);
    }
}
