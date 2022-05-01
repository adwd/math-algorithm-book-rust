use proconio::input;

/// 031 - Taro's Vacation
/// 太郎君の夏休みは N 日間あり、i 日目に勉強すると Ai だけ実力が上がることが知られています。
/// しかし、彼は 2 日連続で勉強したくありません。太郎君が夏休みの間に実力をどれだけ上げられるか、その最大値を求めるプログラムを作成してください。
/// 2 ≤ N ≤ 500000
/// 0 ≤ Ai ≤ 10^9
/// 入力はすべて整数
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", solve(n, &a));
}

fn solve(n: usize, a: &[usize]) -> usize {
    // 勉強した場合、しない場合の実力の最大値を持っておく
    let mut dp = vec![(0, 0); n];
    dp[0].0 = a[0];

    for i in 1..n {
        dp[i].0 = dp[i - 1].1 + a[i];
        dp[i].1 = dp[i - 1].0.max(dp[i - 1].1);
    }

    dp[n - 1].0.max(dp[n - 1].1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, &[2, 5, 3, 3, 1]), 8);
    }
}
