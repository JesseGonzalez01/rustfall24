use std::io::{self, Read, Write, BufReader};
use std::fs::File;

struct Person {
    car: String,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let car = buffer.trim().to_string();
    buffer.clear();

    // print!("How old are you? ");
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut buffer).unwrap();
    // let age = buffer.trim().parse().unwrap();

    let person = Person { car };

    fn create_and_write_to_file() {
        let mut file = File::create("user_info.txt").unwrap();
        writeln!(file, person).unwrap();
    }
    
    fn read_entire_file() {
        let mut file = File::open("user_info.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("File contents:\n{}", contents);
    }



    // println!("Hi {}, you are {} years old!", person.name, person.age);

}

fn main() {
    reading_from_console();
    create_and_write_to_file();
    println!("Reading entire file:");
    read_entire_file();
}