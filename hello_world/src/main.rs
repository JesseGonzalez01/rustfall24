fn pattern_match_simple() {
    let num = 3;
    let letter = match num {
        1 => 'A',
        2 => 'B',
        3 => (
            println("Hey hey you choose number 3 right?");
            'C'
        ),
        _ => '#', // rust will not guess
    };



    println!("{}", letter);
}
fn main() {
    pattern_match_simple();

}