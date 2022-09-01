// 标准库：输入输出库
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // 可变
    let mut guess = String::new();  

    // &表示这个参数是一个引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("Your guessed: {}", guess);
}
