use std::io;

fn main() {
    println!("Enter the position in the fibonacci sequence: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n_fibonacci: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input value. Please enter a number.");
            return;
        }
    };

    let result_sequential :u64 = fibonacci(n_fibonacci, false);
    let result_recursive : u64 = fibonacci_recursive(n_fibonacci);
    println!("The {}-th Fibonacci number \n\t-> (calculated recursively) is: {}\n\t-> (calculated sequentially) is: {}", n_fibonacci, result_recursive, result_sequential);
}

fn fibonacci(n: u32, print_sequence: bool) -> u64{
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    if print_sequence {
        println!(" {a} ");
    }
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
        if print_sequence {
            println!(" {a}");
        }
    }

    if print_sequence {
        println!("The {}-th Fibonacci number is: {}", n, a);
    }
    a
}

fn fibonacci_recursive(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}
