use std::io;

trait IsPrime {
    fn is_prime(&self) -> bool;
}

impl IsPrime for u64 {
    fn is_prime(&self) -> bool {
        if *self <= 1 {
            return false;
        }
        if *self == 2 {
            return true;
        }
        let m = (*self as f64).sqrt() as u64;
        for x in 2..m {
            if *self % x == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {

    let mut n = String::new();
    println!("Please enter the number:");
    io::stdin().read_line(&mut n).expect("Number not entered");
    let n: u64 = n.trim().parse().expect("Please type a number");
    println!("Is the number {} a prime? {}", n, n.is_prime());

}

