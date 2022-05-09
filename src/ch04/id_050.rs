use proconio::input;

/// 050 - Power
fn main() {
    input! {
        a: u64,
        b: u64,
    }

    println!("{}", solve(a, b));
}

fn solve(a: u64, b: u64) -> u64 {
    let m = 1000000007;

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
        assert_eq!(solve(5, 23), 871631629);
        assert_eq!(solve(97, 998244353), 934801994);
    }
}
