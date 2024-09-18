fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 // Guess too high
    } else {
        -1 // Guess too low
    }
}

fn main() {
    let secret_number = 15; // Hard-coded secret number
    let mut guess = 20; // Start with a high guess
    let mut attempts = 0;

    loop {
        attempts += 1;

        // Check the guess
        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Congratulations! You've guessed the correct number: {}", guess);
            break;
        } else if result == 1 {
            println!("Your guess of {} is too high. Guess lower.", guess);
            guess -= 1; // Decrease guess
        } else {
            println!("Your guess of {} is too low. Guess higher.", guess);
            guess += 1; // Increase guess
        }
    }

    println!("It took you {} guesses to find the secret number.", attempts);
}
