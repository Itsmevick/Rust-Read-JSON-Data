use json_reader::UserData;
use serde_json;
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::Path,
    process,
};

fn read_json_data<P: AsRef<Path>>(path: P) -> Result<Vec<UserData>, Box<dyn Error>> {
    let file = File::open(path).unwrap_or_else(|err| {
        eprintln!("Error opening the file");
        eprintln!("Error: {:?}", err);
        process::exit(1)
    });
    let rdr = BufReader::new(file);
    let data = serde_json::from_reader(rdr).unwrap();
    Ok(data)
}

fn main() {
    let data = read_json_data("todos.json").unwrap();
    let UserData {user_id,id,title,completed} = data[0].clone();
    println!("User Id: {}\nID: {}\nTitle: {}\nCompleted: {}", user_id, id, title, completed);
}
