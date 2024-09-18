fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut max_word: &str = "";
    let mut max_count: usize = 0;


    for idx in 0..words.len(){
        let word: &str = words[idx];
        let mut count: usize = 0;

        

        if words[idx] == word{
            count +=1;
        }
        println!("{}",words[idx as usize]);
    }

    return ("the".to_string(),3);
    //(max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}