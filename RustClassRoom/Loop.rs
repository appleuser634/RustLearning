fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 15 {
            break counter * 2;
        }
    };

    println!("Result is {}", result);
}
