use std::fs::File;
use std::io::{Write, Read, BufReader};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::hash::Hash;
use serde::{Deserialize, Serialize};


fn write_phonebook() -> std::io::Result<()> {
    println!("Create file");
    let mut file = File::create("phonebook.json")?;
    file.write_all(b"Yippee ki yay martha flockers")?;
    Ok(())
}

fn read_phonebook() -> std::io::Result<String>{
    println!("Reading phonebook");
    let mut file = File::open("phonebook.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_hashmap_to_json(hm: &mut HashMap<&str, Vec<&str>>) -> std::io::Result<()> {
    let mut file = File::create("phonehash.json")?;
    serde_json::to_writer(file, hm)?;
    Ok(())
}

fn read_json_to_hashmap() -> std::io::Result<HashMap<String, Vec<String>>>{
    // use String to read because the size is not known at compile time.
    let file = File::open("phonehash.json")?;
    let reader = BufReader::new(file);
    let hm_items: HashMap<String, Vec<String>> = serde_json::from_reader(reader)?;
    Ok(hm_items)

}

fn main() -> std::io::Result<()> {
    // reading and writing a text file.
    write_phonebook()?;
    let content: String = read_phonebook()?;
    println!("File content: {}", &content);

    // appending to a text file.
    let mut file = File::options().append(true).open("phonebook.json")?;
    writeln!(&mut file, "\nIs that all you got?")?;
    let content: String = read_phonebook()?;
    println!("File content: {}", &content);

    // reading and writing a hashmap into json with serde_json
    let mut book:HashMap<&str, Vec<&str>> = HashMap::new();
    book.insert("fun", vec!["bar", "baz", "bang"]);
    book.insert("kung", vec!["fu", "fufu"]);
    write_hashmap_to_json(&mut book)?;

    let content = read_json_to_hashmap()?;
    println!("{:?}", content);
    Ok(())
}
