fn main1() {
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

}

//rust中的数据溢出
fn main2()
{
    let x:u8=255;
    let b=x.wrapping_add(20);
    assert_eq!(100u8.saturating_add(1), 101);//调用饱和加法 //输出101
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);//调用饱和加法 输出255,限制最大值
    println!("wrapping_add {}",b);//19  255+20=275-256=19
}

//浮点数精度问题，精度根据选择的位数会发生变化 例如f32 f64 默认使用f64
//浮点数的运算eg  if(0.1+0.2==0.3)会出错有两种解决办法
//if(0.1_64+0.2_f64==0.3_f64);
//(0.1_f64 + 0.2 - 0.3).abs() < 0.00001//abs表述绝对值

//Nan 未定义的数学行为例如-42的算数平方根


//字符 布尔 单元类型
//字符必须单引号  单元类型() 不占空间


//表达式后面不加分号
fn main() {
    assert_eq!(ret_unit_type(), ())
}
fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };

    // if x % 2 == 1 { "odd" } else { "even" }那么这就是表达式函数就有返回值了
}

//函数的参数需要表明类型 函数可以无返回值利用单元类型
//发散函数是永远不返回的函数类似于死循环




