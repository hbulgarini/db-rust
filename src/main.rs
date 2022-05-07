use db_handler::connection::{DBCalls, DBConnection};
use db_handler::query::{DBQuery, Id};
use std::env;

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
    let db_connection = DBConnection::init_db(db_file_name.unwrap());
    let mut query = DBQuery { db_connection };
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
                "add" => query.add(registry),
                "update" => {
                    eprintln!("error: Missing ID or Row parameter");
                    help();
                }
                "show" => {
                    let id_str = &args[3];
                    let id_parsed: u32 = id_str.parse().unwrap();
                    let id = Id { id: id_parsed };
                    query.show(None, &id)
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
                "update" => query.update(id, registry),
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
