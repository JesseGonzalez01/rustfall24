fn main() {
    {
        let s = "hello"; // s is valid from this point forward
        println!("message from stack: {}", s);
    } // s goes out of scope here
    
    // println!("message: {}", s); // This would result in an error
}