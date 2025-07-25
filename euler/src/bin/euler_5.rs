// $2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from $1$ to $20$?

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut result: u64 = 1;
    for i in 1..=20 {
        result = lcm(result, i);
    }
    println!("{}", result);
}
