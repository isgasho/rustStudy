use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess() {
    println!("猜猜看数字");

    let secret_number = rand::thread_rng().gen_range(1,1001);

    loop {
        println!("请输入你的猜测数字");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("faild to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的猜测: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("你猜对了");
                break;
            }
        }
    }
}