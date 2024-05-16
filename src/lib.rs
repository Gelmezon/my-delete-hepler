use chrono::{DateTime, Utc};
use regex::Regex;
use std::time::SystemTime;

/**
 * 格式化时间
 * @param created 创建时间
 * @param format 格式化字符串
 */
fn format_time(created: SystemTime, format: &str) -> String {
    //使用chrono 格式化时间
    // 将SystemTime转换为DateTime<Utc>
    let datetime_utc: DateTime<Utc> = created.into();
    // 将UTC时间转换为东八区时间
    let offset =
        &chrono::FixedOffset::east_opt(8 * 3600).expect("FixedOffset::east out of bounds 时区错误");
    let datetime_east8 = datetime_utc.with_timezone(offset);
    // 格式化为指定的日期格式
    let formatted_date = datetime_east8.format(format).to_string();
    formatted_date
}

/**
 * 正则匹配
 * @param re 正则表达式
 * @param path 文件路径
 */
fn regex_search(re: Regex, path: &str) -> bool {
    let result = re.is_match(path);
    result
}

/**
* 删除文件
* @param path 文件路径

*/
fn remove_file(path: &str) -> () {
    //删除文件
    // let res = std::fs::remove_file(path);
    // match res {
    //     Ok(_) => {
    //         println!("删除成功!");
    //     }
    //     Err(e) => {
    //         println!("删除失败! {:?}", e);
    //     }
    // }
    println!("删除文件: {:?}", path);
    ()
}

/**
 * 清理日志
 * @param conf 配置
 */
pub fn clear_log(conf: ConfigDetail) -> () {
    //清理日志
    let path = conf.path;
    let regex = conf.regex;
    let expire_day = conf.day;

    println!(
        "path: {:?},regex: {:?},expire_day: {:?}",
        path, regex, expire_day
    );

    let input_reg = Regex::new(regex.as_str()).unwrap();
    //获取path目录下所有文件
    let paths = std::fs::read_dir(&path).expect(&format!("path: {:?} 不存在!", &path));

    let mut array = Vec::new();

    for path in paths {
        //如果是文件夹 跳过
        if path.as_ref().unwrap().path().is_dir() {
            continue;
        }
        //获取文件的创建日期
        let metadata = std::fs::metadata(path.as_ref().unwrap().path()).unwrap();
        let modify_time = metadata.modified().unwrap();
        //取得系统时间
        let current_time = SystemTime::now();
        //获取文件创建日期到现在的时间差
        let duration = current_time.duration_since(modify_time).unwrap();
        let day = duration.as_secs() / 60 / 60 / 24;
        //如果文件创建日期小于expire_day天
        if day < expire_day {
            continue;
        }
        // 确保path_option是Some，并且获取内部的PathBuf
        let path_buf = path.unwrap();
        // 获取PathBuf的引用，避免临时值被释放
        let path = path_buf.path();
        // 转换为字符串
        let file_name = path.file_name().unwrap().to_str().unwrap();
        // 匹配文件名
        let is_match = regex_search(input_reg.clone(), path.to_str().unwrap());
        if !is_match {
            continue;
        }

        let time = format_time(modify_time, "%Y-%m-%d");
        //转成utf+8 的 yyyy-mm-dd hh:mm:ss 格式
        println!("匹配到文件 name: {:?},created:{:?}", file_name, time);
        //删除文件
        array.push(path);
    }
    if array.len() == 0 {
        println!("没有匹配到文件!");
        return;
    }
    println!("需要删除的文件为:");
    for path in &array {
        println!("{:?}", path.to_str().unwrap());
    }
    println!("是否删除以上文件? y/n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "y" {
        println!("取消删除!");
        return;
    }
    for path in array {
        remove_file(path.to_str().unwrap());
    }

    ()
}

/**
 * 读取json配置文件
 * @return Vec<ConfigDetail> 配置明细数组
 */
pub fn read_conf_from_json() -> Vec<ConfigDetail> {
    //读取json配置文件
    let path = std::env::current_dir().unwrap().join("config.json");
    println!("path: {:?}", path);
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let conf: serde_json::Value = serde_json::from_reader(reader).unwrap();
    //conf转为数组
    let array = conf.as_array().unwrap();
    let mut res = Vec::new();
    for item in array {
        let path = item["path"].as_str().unwrap();
        let regex = item["regex"].as_str().unwrap();
        let day = item["day"].as_u64().unwrap();

        let config = ConfigDetail::new(path.to_string(), regex.to_string(), day);
        res.push(config);
    }
    res
}
/**
* 配置明细
*/
#[derive(Debug)]
pub struct ConfigDetail {
    //文件路径
    path: String,
    //正则表达式
    regex: String,
    //过期天数
    day: u64,
}

/**
 * 实现ConfigDetail
 */
impl ConfigDetail {
    fn new(path: String, regex: String, day: u64) -> ConfigDetail {
        ConfigDetail { path, regex, day }
    }
}
