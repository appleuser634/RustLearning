// 引数の方は必ず明記しなければならない
// returnで処理を中断して、値を返す事もできる
// 返り値の型は->で指定する
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply((a, b): (i32, i32)) -> i32 {
    a * b
}

fn main() {
    let mut s = add(4, 5);
    s = add(s, 100);
    println!("{}", s);
    let l = (3, 8);
    let m = multiply(l);
    println!("{}", m)
}
