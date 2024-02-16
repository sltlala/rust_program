use std::io;
use std::io::Write;
use std::io::Error;
use std::fs::File;
use std::time::{Instant};
fn main() -> Result<(), Error> {
    println!("Please input your number.");
    // 获取输入的数字
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");
    let i:u32 = number.trim().parse().expect("Please type a number!");

    // 将各运行时间存入Vec中
    let mut times_list: Vec<String> = Vec::new();
    for number in 0..i {
        let time = collect_times(number);
        times_list.push(time);
    }
    // 将结果的集合存入txt文件
    let file_path = "fibonacci_result.txt";
    let mut file = File::create(file_path)?;
    for item in &times_list {
        writeln!(file, "{}", item)?; // 将每个元素写入文件，并添加换行符
    }
    println!("{:?}",times_list);
    Ok(())
}

// 斐波那契数列计算
fn fibonacci(n:u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n -2);
}

// 运行用时统计
fn collect_times(number:u32) -> String{
    let start = Instant::now();
    fibonacci(number);
    //print!("fib({i}): {}",fibonacci(i));
    let end = Instant::now();
    let elapsed = end - start;
    let time = format!("{:?}",elapsed.as_nanos());
    return time
}
