#[derive(Debug, Clone, Copy, PartialEq)]
// 列列挙
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
// 構造体
struct Card {
    // 上で定義した列挙型を使用
    suit: Suit,
    rank: i32,
}

fn main() {
    let suit = Suit::Club; // 列列挙
    let rank = 1;
    let card = Card { suit, rank }; // 構造体のインスタンスを生成
    println!("{:?}", card); // Card { suit: Club, rank: 1 }
}
