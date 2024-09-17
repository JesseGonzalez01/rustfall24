fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut new_word: String = (*s1).clone();
    new_word.push_str(s2);
    new_word
}

fn main() {
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("World!");
    let result: String = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}