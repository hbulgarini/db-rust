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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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
        if self.db_connection.new {
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

    fn generate_person(registry: &str) -> Result<Person,()> {
        let values: Vec<&str> = registry.split(";").collect();
        if let [name, lastname, job_provided] = &values[..] {
            let jobs = job_provided.split("#").collect::<Vec<&str>>().iter().map(|job| {
                let values: Vec<&str> = job.split(",").collect();
                Job {
                    company: values.get(0).unwrap_or(&"").to_string(),
                    from: values.get(1).unwrap_or(&"").to_string(),
                    to: values.get(2).unwrap_or(&"").to_string(),
                    title: values.get(3).unwrap_or(&"").to_string(),
                }
            }).collect::<Vec<Job>>();

            let person = Person {
                name: name.to_string(),
                lastname: lastname.to_string(),
                jobs,
                tech_stack: Vec::new(),
            };

            Ok(person)
        } else {
            eprintln!("Invalid registry!");
            Err(())
        }
    }

    pub fn add(&mut self, registry: &str) {
        let (mut db_updated, last_id) = self.open();

        let next_id = Id { id: last_id.id + 1 };
        if let Ok(new_person) = DBQuery::generate_person(&registry) {
            db_updated.insert(next_id.clone(), new_person);
            self.db_connection.write_to_db(&db_updated);
            self.show(Some(&db_updated), &next_id);
        }
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

    pub fn update(&mut self, id: u32, update: &str) {
        let (mut db, last_id) = self.open();
        if id >= last_id.id + 1 {
            panic!("No existing record with that Id");
        }
        let id = Id { id };
        if let Ok(udpated_row) = DBQuery::generate_person(&update) {
            println!("Updating row with id: {}", &id.id);
            db.insert(id.clone(), udpated_row.clone());
            self.db_connection.write_to_db(&db);
            self.show(Some(&db), &id);
        }
    }

    pub fn delete(&mut self, registry: &str) {
        println!("Delete registry {}", registry);
    }
}
