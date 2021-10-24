fn main() {
    let a = 10;
    let b = a; // 参照を渡すのではなく、値をコピーして代入
    println!("{} {}", a, b);

    let c = 10;
    let c_ref = &c; // 不変参照もコピートレイトが可能　ただし可変参照は一つしか存在できないため、コピートレイトできない
    let c_ref_copy = c_ref;
    print!("{} {} {}", c, c_ref, c_ref_copy);
}
