use crate::connection::{DBCalls, DBConnection};
use codec::{Decode, Encode};
use std::fmt::{Debug, Display, Formatter};
use std::{collections::BTreeMap, io::Read};

#[derive(Encode, Decode, Debug, Clone)]
struct Job {
    company: String,
    from: String,
    to: String,
    title: String,
}

impl Display for Job {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "     Job: Company: {}, From: {}, To: {}, Title: {}",
            self.company, self.from, self.to, self.title
        )
    }
}

type DB = BTreeMap<Id, Person>;

#[derive(Encode, Decode, Debug, Clone)]
enum TechStack {
    Javascript,
    Rust,
    Devops,
}

#[derive(Encode, Decode, Debug, Eq, PartialOrd, Ord, PartialEq, Clone)]
pub struct Id {
    pub id: u32,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Person {
    name: String,
    lastname: String,
    jobs: Vec<Job>,
    tech_stack: Vec<TechStack>,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Name: {}, Last Name: {}", self.name, self.lastname)?;
        for job in &self.jobs {
            write!(f, "\n{}", job)?;
        }
        write!(f, "")
    }
}

pub struct DBQuery {
    pub db_connection: DBConnection,
}

impl DBQuery {
    fn open(&mut self) -> (DB, Id) {
        if self.db_connection.new == true {
            return (DB::new(), Id { id: 0 });
        } else {
            let mut buf: Vec<u8> = vec![];
            self.db_connection.db_file.read_to_end(&mut buf).unwrap();
            let mut input = &buf[..];

            let db = DB::decode(&mut input).unwrap();

            let current_id = db.len();
            return (
                db,
                Id {
                    id: current_id as u32,
                },
            );
        };
    }

    fn generate_person(registry: &String) -> Person {
        let values: Vec<&str> = registry.split(";").collect();
        let name = values[0].to_string();
        let lastname = values[1].to_string();
        let jobs_provided: Vec<&str> = values[2].split("#").collect();
        //let tech_stack:Vec<&str> = values[3].split(",").collect();

        let mut jobs: Vec<Job> = Vec::new();
        let jobs_iter = jobs_provided.iter();

        for job in jobs_iter {
            let values: Vec<&str> = job.split(",").collect();
            let job_entry = Job {
                company: values[0].to_string(),
                from: values[1].to_string(),
                to: values[2].to_string(),
                title: values[3].to_string(),
            };
            jobs.push(job_entry);
        }

        let person = Person {
            name,
            lastname,
            jobs,
            tech_stack: Vec::new(),
        };

        person
    }

    pub fn add(&mut self, registry: &String) {
        let (mut db_updated, last_id) = self.open();

        let next_id = Id { id: last_id.id + 1 };
        let new_person = DBQuery::generate_person(&registry);
        db_updated.insert(next_id.clone(), new_person);
        self.db_connection.write_to_db(&db_updated);
        self.show(Some(&db_updated), &next_id);
    }

    pub fn show_all(&mut self) {
        let (db, _last_id) = self.open();
        for (key, value) in db.into_iter() {
            println!("Id {} {}", key.id, value);
        }
    }

    pub fn show(&mut self, db: Option<&DB>, id: &Id) {
        match db {
            Some(db) => {
                println!("Row Id: {} Data: {:?}", id.id, db.get(&id));
            }
            None => {
                let (db, _last_id) = self.open();
                println!("Row Id: {} Data: {:?}", id.id, db.get(&id));
            }
        }
    }

    pub fn update(&mut self, id: u32, update: &String) {
        let (mut db, last_id) = self.open();
        if id >= last_id.id + 1 {
            panic!("No existing record with that Id");
        }
        let id = Id { id };
        let udpated_row = DBQuery::generate_person(&update);
        println!("Updating row with id: {}", &id.id);
        db.insert(id.clone(), udpated_row.clone());
        self.db_connection.write_to_db(&db);
        self.show(Some(&db), &id);
    }

    pub fn delete(&mut self, registry: &String) {
        println!("Delete registry {}", registry);
    }
}
