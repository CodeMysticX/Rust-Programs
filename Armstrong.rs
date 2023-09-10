fn check_armstrong(num: i32) {
    let mut rs = 0;
    let mut n = num;
    let num_str = num.to_string();
    let len = num_str.len();
    while n > 0 {
        let dig = n % 10;
        rs += i32::pow(dig, len as u32);
        n /= 10;
    }
    if rs == num {
        println!("{} is an Armstrong number.", num);
    } else {
        println!("{} is not an Armstrong number.", num);
    }
}

fn main() {
    let num: i32 = 153;
    check_armstrong(num);
}
