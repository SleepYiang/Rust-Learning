use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

   //创建一个可变的变量 guess
   let mut guess =String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");//防止程序出现崩溃
    let guess: u32=guess.trim().parse().expect("Please type a number!");//将字符串转换为数字
    loop{
        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number){
        Ordering::Less=>println!("Too small!"),
        Ordering::Greater=>println!("Too big!"),
        Ordering::Equal=>{
            println!("You win!");
            break;
            }
        }
    }
    // let x=5;
    // let y=10;
    // println!("x={x},y+2={}",y+2);
}