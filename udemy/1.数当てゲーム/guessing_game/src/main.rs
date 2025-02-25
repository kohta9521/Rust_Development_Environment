use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数当てゲーム!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("数を入力してください。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("入力された値は{}です。", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます。"),
            Ordering::Greater => println!("大きすぎます。"),
            Ordering::Equal => {
                println!("正解です！");
                break;
            }
        }
    }
}