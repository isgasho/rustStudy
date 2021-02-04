use std::collections::HashMap;
use rand::Rng;
mod front;
use front::{front_of_house, eat_at_restaurant, hosting};

extern crate grep;
use grep::Config;
use std::env;
use std::process;

extern crate pkg;

mod study;
use study::{back_of_house, tmps, tmpStruct, tmp2, tmp};

extern crate guess;

fn main() {
    rand_test();
    grep_test();
    front_test();
    init_mod();

    front_test_crate();

    study_test();
    guess::guess();
}

// 同级文件夹 crate导入测速
fn study_test() {
    back_of_house::fix_incorrect_order();

    let food = back_of_house::Breakfast::summer("hello");
    println!("{:?}", food);

    tmps();
    tmpStruct();
    tmp2();
    tmp();
} 

// 同级文件 crate导入测速
fn front_test_crate() {
    front_of_house::hosting::add_to_waitlist();

    eat_at_restaurant();

    hosting::add_to_waitlist();
}

// 引用lib中mod的struct和方法调用
fn init_mod() {
    pkg_test();

    pkg::host::test::go();

    let t = pkg::host::Guess::new(88);
    println!("value is {:#?} \n {}",t,t.value());

    let open = pkg::host::Open::new("Hello Worlds!!!");
    open.write("yes");

    let info = open.read();
    match info {
        Ok(x) => {
            println!("Read Success: {}",x)
        },
        Err(e) => {
            println!("Error {}", e)
        }
    }

    let info2 = open.fastread();
    match info2 {
        Ok(x) => {
            println!("Read Fast Success: {}", x)
        },
        Err(e) => {
            println!("Error {}", e)
        }
    }
}

// 外部pkg导入测试
fn pkg_test() {
    pkg::parseip();
}

// 公共crate导入测试
fn rand_test() {
    let mut map = HashMap::new();
    map.insert(1, 2);    
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("{}", secret_number);
}

// 同级目录导入测试
fn front_test() {
    front::eat_at_restaurant();
}


// 外部crate导入测试
// @Usage: cargo run path Cargo.toml
// @Parms path 搜索的值
// @Parma Cargo.toml 搜索的文件
fn grep_test() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parseing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}