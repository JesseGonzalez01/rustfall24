const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() {
    let mut fahrenheit_temp = 32.0; // Starting temperature in Fahrenheit

    // Convert and print the initial temperature
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{}°F is {:.2}°C", fahrenheit_temp, celsius_temp);

    // Loop to convert the next 5 temperatures
    for _i in 1..=5 {
        fahrenheit_temp += 1.0;
        let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
        println!("{}°F is {:.2}°C", fahrenheit_temp, celsius_temp);
    }
}
