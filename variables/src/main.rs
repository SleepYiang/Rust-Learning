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



}
