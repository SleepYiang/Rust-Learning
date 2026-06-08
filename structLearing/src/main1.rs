fn main()
{
    let wight=20;
    let height=30;
    println!("the area of the rectangle is {}",area(wight,height));
}
fn area(wight:u32,height:u32)->u32
{
    wight*height
}


//第二种计算方法采用元组 fasle
fn main()
{
    let rect1=(20,30);
    println!("the area of the rectangle is {}",area(rect1));
}
fn area(dimensions :(u32,u32))->u32
{
    dimensions.0*dimensions.1//计算矩形面积
}

//第三种结构体
struct Rangle{
    wight:u32,
    height:u32
}
fn main()
{
    let rect2=Rangle{
        wight:20,
        height:30
    };
    println!("the area of the rectangle is {}",area(&rect2));
}
string::from()

let mut items=HashSet::from(["冰","雪"]);


fn area(rectangle:&Rangle)->u32
{
    rectangle.wight*rectangle.height
}

//调试代码生成
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

let s=String::from("hello world");
s.push_str("!!!");
println!("{}",s);

let s=string::from("hello world"); 
let s1=s.clone();
println!("s={},s1={}",s,s1);   