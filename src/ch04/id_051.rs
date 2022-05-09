use proconio::input;

/// 051 - Combination Hard
fn main() {
    input! {
        x: u64,
        y: u64,
    }

    println!("{}", solve(x, y));
}

fn solve(x: u64, y: u64) -> u64 {
    let m = 1000000007;

    let mut fact = vec![0; (x + y) as usize + 1];
    fact[0] = 1;

    for i in 1..=(x + y) as usize {
        fact[i] = (fact[i - 1] * i as u64) % m;
    }

    let ncr = |n: u64, r: u64| -> u64 {
        division(
            fact[n as usize],
            fact[r as usize] * fact[(n - r) as usize] % m,
            m,
        )
    };

    ncr(x + y, y)
}

fn division(a: u64, b: u64, m: u64) -> u64 {
    (a * mod_pow(b, m - 2, m)) % m
}

fn mod_pow(a: u64, b: u64, m: u64) -> u64 {
    let mut p = a;
    let mut answer = 1;

    for i in 0..30 {
        if b & (1 << i) != 0 {
            answer *= p;
            answer %= m;
        }
        p *= p;
        p %= m;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(1, 2), 3);
        assert_eq!(solve(869, 120), 445891023);
    }
}
