//闭包
fn main() {
    use std::mem;
   let x = 1;
   let sum = |y| x + y;   //采用值捕获的方式捕获x的值 //sum中传二则为2

    assert_eq!(3, sum(2));

    let color = String::from("green");
    let print=|| println!("`color`: {}", color);
    print();

}

//代码更新