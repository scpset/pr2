// function
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// test
fn main() {
    let a = 36;
    let b = 60;
    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}
