use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }

    println!("{}", solve(a, b, c));
}

const M: u64 = 998244353;

fn solve(a: u64, b: u64, c: u64) -> u64 {
    f(a) % M * f(b) % M * f(c) % M
}

fn f(x: u64) -> u64 {
    x * (x + 1) / 2 % M
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(1, 2, 3), 18);
        assert_eq!(solve(1000000000, 987654321, 123456789), 951633476);
    }
}
