use std::io::stdin;

// Fibonnacci
fn main() {
    let mut fibonacci_index: String = String::new();
    loop {
        println!("Enter index of the fibonnaccy sequence you want to know:");

        stdin()
            .read_line(&mut fibonacci_index)
            .expect("Failed to read line");

        println!();
        match fibonacci_index.trim().parse() {
            Ok(num) => {
                println!("YOUR wanted fibonacci number is {}", fibonacci_v1(num));
                println!("YOUR wanted fibonacci number is {}", fibonacci_v2(num));
                break;
            }
            Err(_) => continue,
        };
    }
}

fn fibonacci_v1(n: u32) -> u64 {
    let mut first = 0;
    let mut second = 1;

    // Index: 0 1 2 3 4 5 6 7  8 ...
    // Fib:   0 1 1 2 3 5 8 13 21 ...
    if n == 0 {
        return first;
    }
    // let mut sum: u64 = first + second;
    let mut index: u32 = 1;
    let mut sum = first + second;
    while index < n {
        sum = first + second;
        first = second;
        second = sum;

        index += 1;
    }
    return sum;
}

fn fibonacci_v2(n: u32) -> u64 {
    let fibonacci_numbers: [u64; 2] = [0, 1];

    // Index: 0 1 2 3 4 5 6 7  8 ...
    // Fib:   0 1 1 2 3 5 8 13 21 ...
    if n < 2 {
        return fibonacci_numbers[n as usize];
    }

    let mut smaller_previous: u64 = fibonacci_numbers[0];
    let mut bigger_previous: u64 = fibonacci_numbers[1];
    let mut fib: u64 = smaller_previous + bigger_previous;

    for _index in 2..n {
        smaller_previous = bigger_previous;
        bigger_previous = fib;
        fib = smaller_previous + bigger_previous;
    }
    fib
}
