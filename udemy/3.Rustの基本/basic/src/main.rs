fn say_hello() {
    println!("Hello, Rust!");
}

fn add(a: i32, b: i32) -> i32 {
return a + b;
}

fn main() {
    println!("Hello, world!");
    // 関数
    say_hello();
    // 関数の返り値
    let sum = add(1, 2);
    println!("sum: {}", sum);

    let c: i32 = add(1, 2);
    println!("c: {}", c);
}