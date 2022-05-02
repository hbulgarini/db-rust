use std::env;
use db_handler::connection::{DBConnection,DBCalls};
use db_handler::query::{DBQuery};


fn help() {
    println!("usage:
TO DO");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let db_connection = DBConnection::init_db(&args[1]);
    let mut query = DBQuery{ db_connection }; 
    match args.len() {
        3 => {
            let cmd = &args[2];
            
            match &cmd[..] {
                "show" => query.show(),
                _ => {
                    eprintln!("Missing arguments");
                    help();
                },
            }

        },
        // one argument passed
        4 => {
            let cmd = &args[2];
            let registry = &args[3];

            // parse the command
            match &cmd[..] {
                "add" => query.add(registry),
                "delete" => query.delete(registry),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
