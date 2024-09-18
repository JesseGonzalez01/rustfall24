fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 15, 18, 8, 58];

    // Analyze each number
    for &number in &numbers {
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else {
            println!("{}: {}", number, if is_even(number) { "Even" } else { "Odd" });
        }
    }

    // Calculate the sum using a while loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of numbers: {}", sum);

    // Find the largest number
    let mut largest = numbers[0];
    for &number in &numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number: {}", largest);
}
