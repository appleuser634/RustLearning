use std::io;

// if文の記述方法
fn main() {

    println!("Input Number");
    let mut buffer = String::new(); // 標準入力で取得した値を格納するための変数
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Failed to read line.");// 標準入力の値をbuffer変数へ書き込む

    println!("InputNumber = {}",buffer);

    let buffer = buffer.trim(); // 改行文字を削除
    let number: i32 = buffer.parse().unwrap(); // 文字列を数値へ変換
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 2 or 4")
    }
}
