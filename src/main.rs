use db_handler::connection::{DBCalls, DBConnection};
use db_handler::query::DBQuery;
use db_handler::query::DB;
use std::env;
use std::io::Read;

fn help() {
    println!(
        "usage:
TO DO"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let db_file_name = &args.get(1);
    if db_file_name.is_none() {
        help();
        std::process::exit(1);
    };

    // unwrap here is `safe` because of the previuos check
    let mut db_connection = DBConnection::init_db(db_file_name.unwrap());
    let mut content = String::new();
    let db: DB = match db_connection.new {
        true => DB::new(),
        false => {
            db_connection
                .db_file
                .read_to_string(&mut content)
                .expect("can't read file!");
            serde_json::from_str(&content).expect("can't decode json!")
        }
    };

    let mut query = DBQuery { db };
    match args.len() {
        3 => {
            let cmd = &args[2];

            match &cmd[..] {
                "show_all" => query.show_all(),
                _ => {
                    eprintln!("Missing arguments");
                    help();
                }
            }
        }
        // one argument passed
        4 => {
            let cmd = &args[2];
            let registry = &args[3];

            // parse the command
            match &cmd[..] {
                "add" => query.add(&mut db_connection, registry),
                "update" => {
                    eprintln!("error: Missing ID or Row parameter");
                    help();
                }
                "show" => {
                    let id_str = &args[3];
                    let id_parsed: u32 = id_str.parse().unwrap();
                    query.show(id_parsed)
                }
                "delete" => query.delete(registry),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        5 => {
            let cmd = &args[2];
            let id_str = &args[3];
            let id: u32 = id_str.parse().expect("Invalid u32 id");

            let registry = &args[4];
            // parse the command
            match &cmd[..] {
                "update" => query.update(db_connection, id, registry),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
