use proconio::input;

/// 028 - Frog 1
fn main() {
    input! {
        n: usize,
        h: [i64; n as usize],
    }

    println!("{}", solve(n, &h));
}

fn solve(n: usize, h: &[i64]) -> i64 {
    let mut dp = vec![0; n];
    for i in 0..n {
        match i {
            0 => dp[0] = 0,
            1 => dp[1] = (h[0] - h[1]).abs(),
            _ => {
                let v1 = dp[i - 1] + (h[i - 1] - h[i]).abs();
                let v2 = dp[i - 2] + (h[i - 2] - h[i]).abs();
                dp[i] = v1.min(v2);
            }
        }
    }

    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4, &[10, 30, 40, 20]), 30);
        assert_eq!(solve(6, &[30, 10, 60, 10, 60, 50]), 40);
    }
}
