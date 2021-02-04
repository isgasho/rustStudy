use std::env;
use std::fs;
use std::error::Error;

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/*
 *遍历内容的每一行文本。
 *查看这一行是否包含要搜索的字符串。
 *如果有，将这一行加入列表返回值中。
 *如果没有，什么也不做。
 *返回匹配到的结果列表
 */
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// 目前只需知道 Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它。
    let contents = fs::read_to_string(config.filename)?;

    //println!("With texts: \n {}", contents);
    //for line in search(&config.query, &contents) {
    //println!("{}", line);
    //}

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // 因为 run 函数签名中声明成功类型返回值是 ()，这意味着需要将 unit 类型值包装进 Ok 值中。
    // Ok(()) 一开始看起来有点奇怪，不过这样使用 () 是表明我们调用 run 只是为了它的副作用的惯用方式；
    // 它并没有返回什么有意义的值。
    Ok(())
}