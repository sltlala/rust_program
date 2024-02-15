fn main() {
    for i in 0..1000000 {
        println!("fib({i}): {}",fibonacci(i))
    }
}

fn fibonacci(n:u128) -> u128 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n -2);
}
