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

//所有权   性能上栈的性能高于堆的性能
//深拷贝  浅拷贝  值传递 
//返回值的拷贝方式是移动，string 不能能copy 但是返回不能深拷贝