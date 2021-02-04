#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// use std::fmt;
// use std::io;
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn func1() -> fmt::Result<()> {}

// fn func2() -> io::Result {}

// fn func3() -> Result {}

// fn func4() -> IoResult<()> {}

use std::net::{Ipv4Addr, Ipv6Addr};
pub use crate::front_of_house::hosting;

fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn server_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        // 使用以 super 开头的相对路径从父目录开始调用函数
        super::server_order();

        super::demo6();
    }

    fn cook_order() { println!("this is cook order")}

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 枚举如果是公有的，枚举成员默认就是公有的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // error: private field
    // meal.seasonal_fruit = String::from("bulala");

    // enum in mod
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod front_of_house {
    // pub关键字使外部调用可达
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

fn demo6() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(0) => println!("you got it 0!"),
        Some(3) => println!("this is 3"),
        _ => ()
    }

    if let Some(0) = some_u8_value {
        println!("0 is found")
    } else {
        println!("nothing find by you want")
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Tump);
    // match coin {
    //     Coin::Dime => println!("Dime"),
    //     Coin::Quarter(state) => println!("State quater from {:?}!", state),
    //     _ => count += 1
    // }

    if let Coin::Dime = coin {
        println!("found Nickel")
    } else if let Coin::Quarter(state) = coin {
        println!("found Quarter state is {:?}!", state)
    } else {
        count += 1
    }
}

pub fn tmps() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V6);

    let ip1 = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let ip2 = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let lookback = IpAddr2::V6(String::from("::1"));
    let home2 = IpAddr2::V4N(255,255,255,255);
    println!("{:?}", home2);
    
    assert_eq!("hello","hello".to_string());


    let some_num = Some(5);

    let some_str = Some("a string");
    let absent_num: Option<i32> = None;
    println!("{:?}", absent_num);

    // let abc = None;

    let ac = Some(IpAddr2::V4N(1,1,1,1));

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabma)));
    println!("{}", value_in_cents(Coin::Penny));

    let five = Some(5);
    let ssix = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}",five,ssix,none.is_some());

    tongpeifu();
}

// 通配符
// https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html
fn tongpeifu() {
    let some_u8_value = 0;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => println!("other"),
    }
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

#[derive(Debug)]
enum UsState {
    Alabma,
    #[warn(dead_code)]
    Alaska,
    Tump,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("Lucky Quarter {:?}!", state);
            66
        },
    }
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
    V4N(u8,u8,u8,u8),
    V4Raw(Ipv4Addr),
    V6Raw(Ipv6Addr)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn route(ip_type: IpAddrKind) {}

enum IpAddrKind {
    V4,
    V6,
}

#[warn(non_snake_case)]
pub fn tmpStruct() {
    let mut rect1 = Rectangle{width:20,height:66};
    // println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?} {}", rect1,rect1.area());
    rect1.setWidth(100);
    println!("{}",rect1.area());

    let r1 = Rectangle::square(66);
    println!("{}", r1.area())
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width*self.height
    }

    fn setWidth(&mut self, data: u32) {
        self.width  = data
    }

    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle{width:size,height:size}
    }
}

pub fn tmp2() {
    tmp();

    let t1 = String::from("accb c");
    println!("{}", findkey2(&t1));

    let mut user = User {
        email: String::from("382023823@qq.com"),
        user: String::from("li"),
        sign_in_count: 1,
        active: true,
    };

    let user = build_user(String::from("li"),String::from("382023823@qq/com"));

    println!("{}",user.email);

    let user2 = User {
        email: String::from("1"),
        user: user.user,
        active:user.active,
        sign_in_count:user.sign_in_count,
    };

    let user3 = User {
        email: String::from("e"),
        ..user2
    };

    let black = Color(16,16,16);
    println!("{}", black.1);

    let ret1 = (30,50);
    println!("{}", area(ret1))
}

fn area(dimes: (u32,u32)) -> u32 {
    dimes.0 * dimes.1
}

struct Color(i32,i32,i32);

fn build_user(user: String,email: String) -> User {
    User {
        email, // email:email,
        user, // user:user,
        active: true,
        sign_in_count: 1,
    }
}

#[derive()]
struct User {
    email: String,
    user: String,
    sign_in_count: u64,
    active: bool,
}

fn findkey(s: &String) -> usize {
    // let x = s.split(" ");
    // x.0

    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}

fn findkey2(s: &String) -> &str {
    // let x = s.split(" ");
    // x.0

    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i
            return &s[0..i];
        }
    }
    // s.len()
    &s[..]
}

pub fn tmp() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s1: String = String::from("hello");
    let _s2 = s1.clone();
    println!("{}", s1);

    let s3 = 5;
    let s4 = 5.clone();
    println!("{} {}",s3,s4);

    let len = calc(&s1);
    println!("{}", len);

    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("{}", s5);

    let r1 = &s5;
    let r2 = &s5;
    println!("{} {}", r1,r2);
    let r3 = &mut s5;
    println!("{}",r3);
}

fn calc(s:&String) -> usize {
    s.len()
}

fn change(s:&mut String) {
    s.push_str(",ok baby!")
}