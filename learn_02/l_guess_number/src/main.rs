/* 猜数字游戏 */

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数字游戏：数字范围为1-100");

    let secret_number = rand::thread_rng().gen_range(1..101);  //随机生成一个1-100的整数
    loop {
        println!("请输入你猜测的数字");
        let mut guess_number = String::new();  //变量默认不可变，可变对的话在变量名前加mut

        io::stdin()   //处理输入
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        //如果输入的不是整数类型的话，会继续重新输入
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //比较猜测的数字和生成的秘密数字大小
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("你猜测的数字小了"),
            Ordering::Greater => println!("你猜测的数字大了"),
            Ordering::Equal => {
                println!("恭喜你猜对了！");
                break;
            }
        }
    }
}
