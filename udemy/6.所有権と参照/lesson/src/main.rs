use std::rc::Rc;

fn main() {
    // Box<i32>型の値を作成
    let x = Box::new(1);
    // pでポインタを表示
    println!("x: {:p}", x);
    // *で中身を表示
    println!("*x + 2 = {}", *x + 2);

    // Rc<T>型の値を作成
    let a = Rc::new("hello".to_string());
    // 参照カウントを表示
    println!("count: {}", Rc::strong_count(&a));

    {
        let b = Rc::clone(&a);
        println!("a: {:p}", a);
        println!("b: {:p}", b);
        println!("count: {}", Rc::strong_count(&a));
    }
    // 参照カウントを表示
    println!("count: {}", Rc::strong_count(&a));
}
