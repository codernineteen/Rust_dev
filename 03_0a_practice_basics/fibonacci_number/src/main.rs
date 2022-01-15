fn main() {
    fibonacci_sequence();
}

// 1+1 1+2 2+3 3+5 5+8
fn fibonacci_sequence() {
    let mut first_num = 1;
    let mut second_num = 1;
    loop {
        let sum = first_num + second_num;
        let next_num = sum;
        if next_num >= 2 {
            first_num = second_num;
            second_num = sum;
        }
        println!("{}", next_num);
    }
}
