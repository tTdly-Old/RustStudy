use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 其实就是IO流
fn main() {
  println!("猜猜数字");

  let secret_number = rand::thread_rng().gen_range(1..101);
  println!("生成的随机数是：{}", secret_number);
  loop {
    println!("输入你猜的数字");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("错误");

    // let guess: u32 = guess.trim().parse().expect("输入数字！");
    // 直接抓取错误继续运行
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    println!("你猜的数字是：{}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("小了"),
      Ordering::Greater => println!("大了"),
      Ordering::Equal => {
        println!("对了");
        break;
      },
    };
  }
}
