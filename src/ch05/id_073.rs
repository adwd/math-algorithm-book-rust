use proconio::input;

/// 073 - Sum of Maximum
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", solve(n, &a));
}

const M: usize = 1000000007;

fn solve(n: usize, a: &[usize]) -> usize {
    let mut answer = 0;

    let mut power = vec![1; n + 1];
    for i in 1..=n {
        power[i] = (power[i - 1] * 2) % M;
    }

    for i in 1..=n {
        answer += power[i - 1] * a[i - 1];
        answer %= M;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(2, &[3, 5]), 13);
    }
}
