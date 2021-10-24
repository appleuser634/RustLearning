fn main() {
    // Rustは強力な方推論があるが、以下のように明示的に型を指定することもできる
    let a: i32 = 10;
    let b: u32 = 20;
    let c: f32 = 0.;
    let d: &i32 = &50;
    println!("{} {} {} {}", a, b, c, d);

    // Rustはスコープ内で同じ変数名が使いまわせる
    let str_len = String::from("hello world!");
    let str_len = str_len.len();
    println!("{}", str_len);

    // 型変換を明示的に行う場合はasを使用する
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    // 配列
    let list = [1, 2, 3, 4, 5, 6, 7, 8];
    dbg!("{}", &list[0..4]); // スライスを使用して連続した要素を取り出す

    // 数列 1..100などの記述方法でRange型の数列を生成できる
    let mut sum = 0;
    for i in 1..100 {
        sum += i
    }
    println!("{}", sum);

    // 文字列
    let s = String::from("Hello!");
    let n = s.len();
    println!("{}", n);
}
