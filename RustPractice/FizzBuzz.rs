fn fizz_buzz(n:&i32) -> &str {

    let mut s = "nothing...";

    if n % 15 == 0 {
        s = "FizzBuzz!";
    }
    else if n % 3 == 0 {
        s = "Fizz!";
    }
    else if n % 5 == 0 {
        s = "Buzz!";
    }
    
    return &s;
}


fn main() {
    let n_list = 1..100;
    for n in n_list {
        let result = fizz_buzz(&n);
        println!("{} is {}",n,result);
    }
}
