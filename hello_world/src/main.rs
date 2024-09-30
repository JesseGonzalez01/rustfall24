use std::io::{self, Read, Write};
use std::fs::File;

struct Person {
    car: String,
}

fn reading_from_console() -> Person {
    let mut buffer = String::new();

    print!("What's your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let car = buffer.trim().to_string();

    Person { car }
}

fn create_and_write_to_file(person: &Person) {
    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "{}", person.car).unwrap();
}

fn read_entire_file() {
    let mut file = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn main() {
    let person = reading_from_console();
    create_and_write_to_file(&person);
    println!("Reading entire file:");
    read_entire_file();
}
