// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main() {
//     first_word(s:"test")；
//     let test=01;
// }


//元组
//let(i32,u8,f64)
struct  User{
    username:String,
    email:String,
    active:bool,
    single_num:i32
}
fn main(){
    let user1=User{
        username:String::from("abc"),
        email:String::from("1234565@163.com"),
        active:true,
        single_num:1
    };
    let mut user2=User{
        username:String::from("abc"),
        ..user1
    };
    println!("struct1 {}",user1.active);
    println!("struct2 {}",user2.single_num);

    user2.active=false;
    println!("struct {}",user2.active);
}