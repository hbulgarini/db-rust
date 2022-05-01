use std::env;
use std::fs::{File};
use std::io::Write;
use db_handler::init::{init_db};
use db_handler::crud::add;

fn help() {
    println!("usage:
TO DO");
}



fn main() {

    let args: Vec<String> = env::args().collect();
    println!("args: {:?}",args);

    let db = init_db(&args[1]);
    println!("args leng: {}",args.len());
    match args.len() {
        3 => {
            println!("Try passing some arguments!");
            help();
        },
        // one argument passed
        4 => {
            let cmd = &args[2];
            let registry = &args[3];
            println!("CMD: {:?}", &cmd[..]);
            // parse the command
            match &cmd[..] {
                "add" => add(db,registry),
                "delete" => println!("To delete a new registry in the DB"),
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
