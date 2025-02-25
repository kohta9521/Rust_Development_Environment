fn main() {
    // let mut v1 = vec![1, 2, 3];
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1[0] : {:p}", &v1[0]);

    // println!("v1 len: {}", v1.len());
    // println!("v1 capacity: {}", v1.capacity());

    // v1.push(4);
    // println!("v1 ptr: {:?}", v1.as_ptr());
    // println!("v1 len: {}", v1.len());
    // println!("v1 capacity: {}", v1.capacity());

    // println!("v1 ptr: {:?}", v1.as_ptr());
    // let v2: Vec<i32> = v1.clone();
    // println!("v2 ptr: {:?}", v2.as_ptr());
    // println!("v2 ptr: {:?}", v1.as_ptr());
    // println!("v2 len: {}", v2.len());
    // println!("v2 capacity: {}", v2.capacity());

    let s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    let (s: String, s1: String, s2: String)  = concat(s1, s2);
    println!("{}", s);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}


fn concat(a: String, b: String) -> (String, String, String) {
    let c: String = format!("{}, {}", a, b);
    (a, b, c)
}