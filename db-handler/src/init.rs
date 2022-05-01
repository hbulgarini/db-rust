use std::fs::File;

pub fn init_db(db_name: &String) -> File {

    let db_name = format!("{}.db",db_name);
    println!("Initialiazing DB {}...",db_name);
    let file = File::create(db_name).unwrap();
    file
}
