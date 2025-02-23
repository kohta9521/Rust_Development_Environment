fn main() {
    let s1: String = String::from("Hello, world");

    let mut a: [i32, 3] = [0, 1, 2];
    let b: [i32, 3] = [0; 3];
    a[1] = b[1];
    b[2] = a[2];
    println!("{:?}", &a[1..3]);
}