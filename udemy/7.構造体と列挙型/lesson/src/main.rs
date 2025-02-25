struct Rectangle {
    width: u32,
    height: u32,
}

// メソッドは関数と似ているが、自身のフィールドの値を変更することができる
impl Rectangle {
    // メソッドはselfを引数に取る
    // selfは共有参照
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 型関連関数はselfを引数に取らない
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

fn main() {
    // 実際の値のことをインスタンスという
    let height: u32 = 5;
    // インスタンスを作成
    let mut rectangle = Rectangle {
        width: 10,
        height,
    };
    // インスタンスのフィールドにアクセス
    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);

    // インスタンスのフィールドを変更
    rectangle.height = 10;
    println!("height: {}", rectangle.height);

    // メソッドを呼び出す
    println!("area: {}", rectangle.area());

    // 型関連関数を呼び出す
    let rectangle = Rectangle::new(10, 5);
    println!("width: {}", rectangle.width);
    println!("height: {}", rectangle.height);
}
