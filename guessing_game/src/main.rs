use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

   //创建一个可变的变量 guess
   let mut guess=String::new();

    io::stdin()
        .read_line(&mut guess)//打
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let x=5;
    let y=10;
    println!("x={x},y+2={}",y+2);
}