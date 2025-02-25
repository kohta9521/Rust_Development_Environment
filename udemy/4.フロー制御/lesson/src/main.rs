fn main() {
    // loop
    // let mut cnt: i32 = 0;
    // loop {
    //     println!("{}", cnt);
    //     if cnt == 10 {
    //         break;
    //     }
    //     cnt += 1;
    // }

    // while
    // let mut cnt: i32 = 0;
    // while cnt <= 10 {
    //     println!("{}", cnt);
    //     cnt += 1;
    // }

    // for
    for i in [1, 2, 3] {
        println!("{}", i);
    }

    // range
    let r = 1..10;
    for i in r {
        println!("{}", i);
    }
}
