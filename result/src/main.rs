use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use std::fs;
fn main1()
{

    //rust中的出错误类型 1、panic!终止当前线程抛出异常（需要match考虑所有的结果和分支）
    //  2、unwarp 通过就执行Result中的ok分支不通过直接panic      eg  let f = File::open("hello.txt").unwrap();
    //  3、expect 和unwarp功能相同但可以再内容后面添加报错信息 eg let f= File::open("hello.txt").expect("Failed to open hello.txt");
    //
    //
    //panic!("crash and burn")

    //文件操作方法一
    let mut file=File::create("test.txt").expect("创建文件失败");
    file.write_all("hello rust".as_bytes()).expect("写入失败");
    

}
//学习

fn main() -> Result<(), Box<dyn Error>> {
    // 一行完成写入
    fs::write("hello.txt", "hello")?;
    // 一行完成读取
    let content = fs::read_to_string("hello.txt")?;
    
    println!("{}", content);
    Ok(())
}