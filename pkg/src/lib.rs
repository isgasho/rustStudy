#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::net::IpAddr;

pub fn parseip() {
    // let home:IpAddr = "127.0.0.1".parse().unwrap();
    let home:IpAddr = "127.0.0.1".parse().unwrap();
    // let x = match home {
    //     Ok(info) => info,
    //     Err(error) => {
    //         println!("error {:?}", error)
    //     }
    // };

    println!("123 {}", home);
}

pub mod host {
    pub mod test {
        pub fn go() {
            println!("host test go now");
        }
    }

    #[derive(Debug)]
    pub struct Guess {
        value: i32,
        pub text: String,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100");
            }

            Guess {
                value,
                text: String::from("Hello world!"),
            }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    #[derive(Debug)]
    pub struct Open {
        path: String
    }

    use std::io;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;

    impl Open {
        pub fn new(path: &str) -> Open {
            Open {
                path: String::from(path)
            }
        }

        pub fn write(&self,word: &str) {
            let mut f = File::create("hello.txt").unwrap();
            f.write(format!("{} {}",self.path,word).as_bytes()).unwrap();
        }

        pub fn read(&self) -> Result<String, io::Error> {
            // let f = File::open("hello.txt");
            // let mut f = match f {
            //     Ok(file) => file,
            //     Err(e) => Err(e),
            // };

            // unwrap 调用 panic! 时提供的错误信息：
            // let mut f = File::open("hello.txt").unwrap();

            // 使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
            // expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏。expect 用来调用 panic! 的错误信息将会作为参数传递给 expect ，而不像unwrap 那样使用默认的 panic! 信息。它看起来像这样：
            // let mut f = File::open("hello.txt").expect("Failed to opening hello.txt");

            // Result 值之后的 ? 被定义为与示例 9-6 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式
            // File::open 调用结尾的 ? 将会把 Ok 中的值返回给变量 f。如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者
            let mut f = File::open("hello.txt")?;

            let mut s = String::new();

            // match f.read_to_string(&mut s) {
            //     Ok(_) => Ok(s),
            //     Err(e) => Err(e)
            // };

            // println!("Read hello.txt is {}", s)

            f.read_to_string(&mut s)?;
            Ok(s)
        }

        pub fn fastread(&self) -> Result<String,io::Error> {
            fs::read_to_string("hello.txt")
        }
    }
}