fn check_prime(num: i32) {
    let mut rs = 0;
    let mut i = 2;
    let n = (num as f64).sqrt() as i32; 

    if num <= 0 {
        println!("{} is not a prime number. ", num);
        return;
    }

    while i <= n {
        if num % i == 0 {
            rs += 1;
            break;
        }
        i += 1;
    }

    if rs == 0 {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number. ", num);
    }
}

fn main() {
    let num: i32 = 51;
    check_prime(num);
}
