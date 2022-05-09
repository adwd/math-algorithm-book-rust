use proconio::input;

/// 049 - Fibonacci Easy (mod 1000000007)
fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn solve(n: usize) -> u64 {
    let mut a = vec![1; n + 1];
    for i in 3..=n {
        a[i] = (a[i - 1] + a[i - 2]) % 1000000007;
    }
    a[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(6), 8);
        assert_eq!(solve(3), 2);
        assert_eq!(solve(8691200), 922041576);
    }
}
