fn main() {
    let x = 5; // xを5という値に束縛

    let x = x + 1; // xを覆い隠す

    {
        let x = x * 2;
        println!("The value of x int the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}