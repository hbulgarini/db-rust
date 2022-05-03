use db_handler::connection::{DBCalls, DBConnection};
use db_handler::query::DBQuery;
use std::env;

fn help() {
    println!(
        "usage:
TO DO"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let db_connection = DBConnection::init_db(&args[1]);
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
                "show" => println!(
                    "Add this function by Id. query.show function should recieve an Option<DB>"
                ),
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
            let id: u32 = id_str.parse().unwrap();

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
