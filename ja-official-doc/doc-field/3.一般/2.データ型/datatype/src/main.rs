use std::io;

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0;
    println!("{}", x);
    println!("{}", y);

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    let remainder = 43 % 5;

    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", floored);
    println!("{}", remainder);

    // 論理値型
    let t = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    // 文字列型
    let c: char = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😄';
    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);


    // 複合型
    // タプル型：複数の型のなんらかの値を1つの複合型にまとめ上げる手段
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // 配列型：複数の値のコレクションを得ることが可能
    // タプルと異なり、配列の全要素は、同じ型でなければならない
    let a = [1, 2, 3, 4, 5];
    // let months = ["January", "Feburary", "March", "April", "May", "June", "July", "August", "September", "October", "November", "Decenber"];
    let first = a[0];
    println!("{}", first);

    // 配列要素への無効なアクセス
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index:usize = index
        .trim()
        .parse()
        .expect("Index enterd was not a number");
    let alement = a[index];
    println!("The value of the element at index {} is: {}", index, element);
}