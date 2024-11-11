fn main() {
    // let x = 2.0; // f64

    // let y: f32 = 3.0; // f32

    // let t = true;

    // let f: bool = false;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", x);
    println!("The value of y is: {}", z);

    // let a = [1, 2, 3, 4, 5];
    let a: [i32, 5] = [1. 2. 3. 4. 5];
    println!("The value of a[0] is: {}", a[0]);
}
