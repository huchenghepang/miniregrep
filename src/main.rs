use std::{env,process};

use minigrep::{Config,run};

fn main() {
    let config= Config::new(&env::args().collect());
    println!("要查询的内容是: {:?}", config.query);
    println!("文件路径是: {:?}", config.file_path);

    // println!("开始打开文件...");
    // let file_content = fs::read_to_string(config.file_path).expect("无法打开文件");
    // println!("文件内容是: \n{}", file_content);
    if let Err(e) = run(config){
        println!("发生错误: {:?}", e);
        process::exit(1)
    }
}


#[allow(dead_code)]
fn parse_config(args: &Vec<String>)->Config{
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



