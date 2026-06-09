//引用的相关内容
// fn main(){
//     let s1=String::from("hello");
//     let len=calculate_length(&s1);
//     println!("The length of '{}' is {}",s1,len);
// }
// fn calculate_length(s:&String)->usize
// {
//     s.len()
// }


//引用内容改变
fn main()
{
    let mut s2=String::from("hello");
    change(& mut s2);
    println!("The length of '{}' is {}",s2,s2);
    let reference_to_nothing = denger();
}
fn change(some_string:&mut String)
{
    some_string.push_str(", world");
}

//悬垂引用（不可以返回引用类型）
fn denger() ->String{
    let s = String::from("hello");
    s
}