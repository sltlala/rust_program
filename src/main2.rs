fn main() {              // 程序入口
    let mut x: i32 = 27;  // 可变变量绑定
    print!("{x}");       // 与 printf 类似的输出宏
    while x != 1 {       // 表达式周围没有括号
        if x % 2 == 0 {  // 与其他语言类似的数值计算
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    print!("\n");
    println!("finish!!!");
}