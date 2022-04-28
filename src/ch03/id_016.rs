use proconio::input;

/**
  016 - Greatest Common Divisor of N Integers
  N 個の正の整数 A1,A2,…,AN の最大公約数を求めてください。
*/
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    println!("{}", solve(n, a));
}

fn solve(n: usize, mut a: Vec<u64>) -> u64 {
    for i in 0..n - 1 {
        let g = gcd(a[i], a[i + 1]);
        a[i + 1] = g;
    }
    a[n - 1]
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, vec![12, 18, 24]), 6);
    }
}
