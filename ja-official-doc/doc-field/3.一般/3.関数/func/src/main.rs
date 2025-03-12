fn main() {
    println!("Hello, world!");
    another_function(5);

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("the value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is : {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);

    // Rustは式指向言語
    // 文とはなんらかの動作をして値を返さない命令 ;がつく？
    // 式とは結果値に評価される
}

// 戻り値
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
