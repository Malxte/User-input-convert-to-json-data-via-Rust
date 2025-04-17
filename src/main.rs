// Open, create and edit files
use std::fs::File;
// Import from standart library
use std::io::{self, BufReader, BufWriter, Result};
// Make work with json data easier
use serde::{Deserialize, Serialize};
use serde_json;

// Make a structure for adding a user to json file
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Person {
    // Define that name would be a String
    name: String,
    // Define that age would be a u32 (positive number with 32 bits)
    age: u32,
}

// The function read the json file
fn read_json(file_path: &str) -> Result<Vec<Person>> {
    // Open the json file
    let file = File::open(file_path)?;
    // Read the json file
    let reader = BufReader::new(file);
    let persons: Vec<Person> = serde_json::from_reader(reader)?;
    Ok(persons)
}

// The function write the json file
fn write_json(file_path: &str, persons: &[Person]) -> Result<()> {
    // Open the json file
    let file = File::create(file_path)?;
    // Write the json file
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, persons)?;
    Ok(())
}

// The function add a user to json data
fn add_user() -> (String, u32) {
    // Create the name and age as mutable
    let mut name = String::new();
    let mut age = String::new();

    // Listen and make a expect of the user name
    println!("What is your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Fehler beim Laden der Eingabe");

    // Listen and make a expect of the user age
    println!("How old are you?");
    io::stdin()
        .read_line(&mut age)
        .expect("Fehler beim Laden der Eingabe");

    // Trim the name and age
    let name = name.trim().to_string();
    let age: u32 = age.trim().parse().expect("Bitte eine gültige Zahl eingeben");

    (name, age) // Response of name and age

}

// The function checks if the entered username is already taken
fn user_check(name: String, age: u32) {
    // create a const with the file path to json
    const FILE_PATH: &str = "src/json/user-data.json";

    // Try to read the json file
    let mut persons = match read_json(FILE_PATH) {
        Ok(data) => data,
        Err(_) => Vec::new(), // If the list wont exist make a new
    };

    // Check if the user is in json file
    if persons.iter().any(|p| p.name == name) {
        println!("The username {} is already used.", name);
        main();
    } else {
        // Add new person to list
        let person = Person { name: name.clone(), age };
        persons.push(person);

        // Write the update list in json file
        write_json(FILE_PATH, &persons).expect("Mistake by writing the json file");

        println!("User added!");
    }
}

// The function define name and age with the add_user() function and start the user check
fn main() {
    // Add a user and check if he is in json file
    let (name, age) = add_user(); // Worth from add_user()
    user_check(name, age); // User check of add_user()
}
