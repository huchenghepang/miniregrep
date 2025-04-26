use std::{env, error::Error, fs};
/// # 运行读取文件的主程序
/// 
/// ## 读取文件的内容并打印到控制台
pub fn run (config:Config)->Result<(),Box<dyn Error >>{
    let file_content = fs::read_to_string(config.file_path)?;
    println!("文件内容是: \n{}", file_content);
    let result = if config.case_sensitive {
        search_insensitive(&config.query, &file_content)
        
    }else{
        search(&config.query, &file_content)
    };
    println!("找到了如下内容:\n");
    for line in result{
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
   pub query: String,
   pub file_path: String,
   pub case_sensitive: bool,
}
/// # 配置结构体
/// 
impl Config {
   pub fn new(args:&Vec<String>)->Config{
        let query = if let Some(x) = args.get(1) {
            x.clone()
        }else{
            panic!("请输入要查询的内容");
        };  
        let file_path = if let Some(x) = args.get(2){
            x.clone()
        }else{
            panic!("请输入要查询的文件路径");
        };
        let case_sensitive = env::var("INGORE_CASE").is_ok();
        Config {
            query,
            file_path,
            case_sensitive
        }
    }
}


/// 搜索字符串函数
///
/// # Arguments
///
/// * \`query\` - 搜索指定的字符串
/// * \`content\` - 要搜索的内筒
///
/// # Example
///
/// ```rust
/// use minigrep::search;
/// search("hello", "hello,world");
/// ```
///
/// # Returns
///
/// ["hello"]
pub fn search<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

pub fn search_insensitive<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}

/// # 测试函数
/// 
/// ## 测试函数的功能
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query:&str = "hello";
        let content:&str = "hello,world";
        assert_eq!(vec!["hello"],search(query,content));
    }

    #[test]
    #[should_panic]
    fn fail_one_result(){
        let query:&str = "hello";
        let content:&str = "hello,world";
        assert_eq!(vec!["1,2","1,2,4"],search(query,content));
    }

    #[test]
    fn case_insensitive() {
        let query:&str = "hello";
        let content:&str = "Hello,world";
        assert_eq!(vec!["Hello,world"],search_insensitive(query,content));
    }

}