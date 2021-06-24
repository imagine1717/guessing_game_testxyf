// use 引入当前作用域
// std 称之为 “标准库”
// io 输入/输入库
use std::io;
// 比较函数
use std::cmp::Ordering;

// 产生一个随机数
use rand::Rng;

fn main() {
    println!("猜数字！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // println!("密码数字是：{}", secret_number);

        println!("请输入要猜的数字.");

        // mut 可变，rust 默认是不可变，guess 是可变的变量
        // ::new 调用String关联的函数··
        let mut guess = String::new();

        // 接受输入的信息，并保存到guess中。
        io::stdin().read_line(&mut guess).expect("获取行失败！");

        // let guess: u32 = guess.trim().parse()
        //     .expect("请输入一个数字类型！");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜：{}", guess);

        // guess 和 secrect_number  比较大小
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了点!"),
            Ordering::Greater => println!("大了点!"),
            Ordering::Equal => {
                //猜对了退出循环·
                println!("猜对了!");
                break;
            }
        }
    }
}
