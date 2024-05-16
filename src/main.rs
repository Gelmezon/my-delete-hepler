

fn main() {
    //获取当前目录
    let conf = my_delete_hepler::read_conf_from_json();
    for item in conf {
        my_delete_hepler::clear_log(item);
    }
    println!("完成!");
}
