const MAX_POINTS: u32 = 100_000;
const FIBONACCI_TEST_NUMBER: u8 = 8;

fn main() {
    let mut x = 5u64;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The constant is: {}", MAX_POINTS);
    // Fibonacci test
    println!(
        "The value of the {}th Fibonacci number is {}",
        FIBONACCI_TEST_NUMBER,
        naive_fibonacci(FIBONACCI_TEST_NUMBER),
    );
    // Fahrenheit to Celsius
    let f_test = 32.;
    println!("{}F = {}C", f_test, fahrenheit_to_celsius(f_test));
    // Print the Twelve Days of Christmas song
    twelve_days_of_christmas();
}

fn naive_fibonacci(n: u8) -> u64 {
    if n == 0 || n == 1 {
        n as u64
    } else {
        naive_fibonacci(n - 1) + naive_fibonacci(n - 2)
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.) * (5. / 9.)
}

fn twelve_days_of_christmas() {
    const DAYS_OF_CHRISTMAS: usize = 12;
    const VERSES: [&str; DAYS_OF_CHRISTMAS] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];
    const ORDINALS: [&str; DAYS_OF_CHRISTMAS] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for day in 0..DAYS_OF_CHRISTMAS {
        println!(
            "On the {} day of Christmas my true love sent to me",
            ORDINALS[day],
        );
        for verse in (DAYS_OF_CHRISTMAS - day - 1)..DAYS_OF_CHRISTMAS {
            println!("{}", VERSES[verse]);
        }
    }
}
