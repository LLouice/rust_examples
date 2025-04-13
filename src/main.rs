use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object.
    let p: Person = serde_json::from_str(data)?;

    // Serialize the Person object back to JSON
    // Create and write directly to the file using serde_json::to_writer_pretty
    let mut file = File::create("../Person.json")?;
    serde_json::to_writer_pretty(&mut file, &p)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    // load the Person.json from File
    let file = File::open("../Person.json")?;
    let p: Person = serde_json::from_reader(file)?;
    println!("{:#?}", p);


    Ok(())
}

fn main() {
    typed_example().expect("typed_example failed!");
}
