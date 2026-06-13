//match学习
#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn cents_in_value(value:u8)->Option<Coin>{
    match value{
        1=>Some(Coin::Penny),
        5=>Some(Coin::Nickel),
        10=>Some(Coin::Dime),
        25=>Some(Coin::Quarter),
        _ =>None,
    }
}


fn main()
{
    let x=Coin::Penny;
    let y=value_in_cents(x);
    println!("result{}",y);
    let z:u8=5;
    let resylt=cents_in_value(z);
    //println!("result1{}",resylt);//错误使用方法枚举类型不可以直接打印
    println!("result1{:?}",resylt);//配合debug模式下的：？完成打印
    //方法二
    match cents_in_value(z){
        Some(coin)=>println!("result2{}",value_in_cents(coin)),
        None=>println!("无效字符"),
    }
}