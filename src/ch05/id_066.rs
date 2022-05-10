use proconio::input;

/// 066 - Three Cards
fn main() {
    input! {
        n: u64,
        k: u64,
    }

    println!("{}", solve(n, k));
}

fn solve(n: u64, k: u64) -> u64 {
    // 余事象
    let mut ac = 0;

    for a in 1..=n {
        for b in 1.max(a - (k - 1))..=n.min(a + (k - 1)) {
            for c in 1.max(a - (k - 1))..=n.min(a + (k - 1)) {
                if (b as i64 - c as i64).abs() <= (k - 1) as i64 {
                    ac += 1;
                }
            }
        }
    }

    n * n * n - ac
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, 1), 24);
    }
}
