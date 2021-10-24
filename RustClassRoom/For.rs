fn main() {
    // 配列をイテレーターへ変換して、要素を取り出しながらループできる
    let a = [1, 3, 5, 7, 10];
    for i in a.iter() {
        println!("number is {}", i);
    }

    // 以下のように数列を生成してループする事もできる
    // for i in 1..100 {
    //     println!("number is {}", i);
    // }
}
