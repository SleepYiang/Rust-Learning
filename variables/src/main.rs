fn main() {
    let mut x = 5;//mut 修饰可变的变量内容
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //定义常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    //rust中内容的遮蔽
    let v = 5;

    let v = v + 1;

    {
        let v = v * 2;
        println!("The value of v in the inner scope is: {}", v);
    }
    println!("The value of v is: {}", v);
    
    //遮蔽可以同名不同类型
    let spaces = "   ";
    let spaces = spaces.len();

    //*****************************数据类型*****************************
    let tup: (i32, f64, u8) = (500, 6.4, 1);//元组

    let x: (i32, f64, u8) = (500, 6.4, 1);//类似数组下标给值
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //数组
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    //i32 是每个元素的类型。分号之后，数字 5 表明该数组包含 5 个元素
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    //数组下标访问
    let first = a[0];
    let second = a[1];

    //更新定义函数
fn another_function() {
    println!("Another function.");
}

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
//test

}
