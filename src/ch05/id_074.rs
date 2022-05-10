use proconio::input;

/// 073 - Sum of Maximum
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    println!("{}", solve(n, &a));
}

const M: usize = 1000000007;

fn solve(n: usize, a: &[i64]) -> i64 {
    let mut answer = 0;
    let nn = n as i64;

    for i in 1..=n {
        answer += a[i - 1] * (-nn + 2 * (i as i64) - 1);
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, &[1, 3, 5]), 8);
    }
}
