use proconio::input;

/// 030 - Knapsack 1
/// N 個の品物があります。 品物には 1,2,…,N と番号が振られています。 各 i(1 ≤ i ≤ N) について、品物 i の重さは wi で、価値は viです。
/// 太郎君は、N 個の品物のうちいくつかを選び、ナップサックに入れて持ち帰ることにしました。
/// ナップサックの容量は W であり、持ち帰る品物の重さの総和は W 以下でなければなりません。
/// 太郎君が持ち帰る品物の価値の総和の最大値を求めてください。
/// 入力はすべて整数である。
/// 1 ≤ N ≤ 100
/// 1 ≤ W ≤ 10^5
/// 1 ≤ wi ≤ W
/// 1 ≤ vi ≤ 10^9
fn main() {
    input! {
        n: usize,
        w: u64,
        wv: [(usize, i64); n],
    }

    println!("{}", solve(n, w, wv));
}

fn solve(n: usize, w: u64, wv: Vec<(usize, i64)>) -> i64 {
    let mut dp = vec![vec![std::i64::MIN; w as usize + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0_usize..=w as usize {
            if j < wv[i - 1].0 {
                dp[i][j] = dp[i - 1][j];
            }
            if j >= wv[i - 1].0 {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1);
            }
        }
    }

    let mut result = 0;
    for i in 0..=w {
        result = result.max(dp[n][i as usize]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, 8, vec![(3, 30), (4, 50), (5, 60)]), 90);
    }
}
