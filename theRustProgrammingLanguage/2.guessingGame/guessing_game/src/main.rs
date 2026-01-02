use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数当てゲームスタート！");
    let secret_number = rand::rng().random_range(1..101);
    println!("秘密の数字は: {}", secret_number);

    loop {
        println!("ほら、予想の数字を入力してみて！");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想の数字は: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます！"),
            Ordering::Greater => println!("大きすぎます！"),
            Ordering::Equal => {
                println!("正解です！");
                break;
            }
        }
    }

}