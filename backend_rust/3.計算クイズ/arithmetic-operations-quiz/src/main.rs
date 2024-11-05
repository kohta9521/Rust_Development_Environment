use rand::Rng;

fn main() {
    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);

    println!("{} + {} = ??", op1, op2);
    println!("??の値を入力してください:");

    // ユーザーからの回答を保持する変数
    let mut ans_input = String::new();

    // 標準入力から１業取得し、ans_inputに格納
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_inputからtrim()で改行を取り除きparse()で整数(u32)に変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input); // cargo runした後に入力した値が確認できる
    if dbg!(ans_input == op1 + op2) {
        println!("正解です！");
    } else {
        println!("不正解です！");
    }

    // 減算
    let op1 = rand::thread_rng().gen_range(0..100);
    let op2 = rand::thread_rng().gen_range(0..100);

    println!("{} - {} = ??", op1, op2);
    println!("??の値を入力してください:");

    // ユーザーからの回答を保持する変数
    let mut ans_input = String::new();

    // 標準入力から１業取得し、ans_inputに格納
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_inputからtrim()で改行を取り除きparse()で整数(u32)に変換
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input); // cargo runした後に入力した値が確認できる
    if dbg!(ans_input == op1 - op2) {
        println!("正解です！");
    } else {
        println!("不正解です！");
    }

    println!("i32が扱えるデータ範囲: {} ~ {}", i32::MIN, i32::MAX);
    println!("u32が扱えるデータ範囲: {} ~ {}", u32::MIN, u32::MAX);
}