//枚举类型的相关内容
// enum IpAdressKey{
//     V4,
//     V6,
// }



//关联结构体使用
// struct IPadd{
//     kind:IpAdressKey,
//     add:String,

// }
//使用枚举变量（创建实例）
// let four=IpAdressKey::V4;
// let six=IpAdressKey::V6;
//自定义函数访问
// fn router(ip_type: ipAdressKey){
//     kind:ipAdressKey,
//     add:String,
// }

// fn router(ip: IPadd) {
//     match ip.kind {
//         IpAdressKey::V4 => println!("路由IPv4地址: {}", ip.add),
//         IpAdressKey::V6 => println!("路由IPv6地址: {}", ip.add),
//     }
// }

// fn main()
// {
//     let ip1=IPadd{
//         kind:IpAdressKey::V4,
//         add:String::from("127.0.0.1"),
//     };

//     let ip2=IPadd {
//          kind:IpAdressKey::V6,
//           add:String::from("::1"),
//         };

//     router(ip1);
//     router(ip2);
// }


//第二种枚举实现
#[derive(Debug)]
enum IpAdressKey{
    V4(String),
    V6(String),
}

fn main()
{
   let home=IpAdressKey::V4(String::from("127.0.0.1"));
   let loop_CallBack=IpAdressKey::V6(String::from("::1"));
   println!("{:?}",home);
   println!("{:?}",loop_CallBack)
}
//test