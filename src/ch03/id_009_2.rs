use proconio::input;

/// 009 - Brute Force 2
/// この問題は 2.4 節（計算量と全探索）と 3.7 節（動的計画法）両方で扱います。
/// 全探索で解いても 1000 点中 500 点しか得られず、満点（AC）にならないことに注意してください。
/// （本に記されている通り、一部の大きいケースでは現実的な時間で答えが求まらないからです）
/// N 枚のカードが横一列に並べられています。左から i 番目 (1≤i≤N) のカードには整数 Ai が書かれています。
/// カードの中からいくつかを選んで、合計がちょうど S となるようにする方法はありますか。
/// 1 ≤ N ≤ 60
/// 1 ≤ Ai ≤ 10000
/// 1 ≤ S ≤ 10000
/// 入力はすべて整数
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    // println!("{}", solve(n, s, &a));
    println!("{}", if solve(n, s, &a) { "Yes" } else { "No" });
}

fn solve(n: usize, s: usize, a: &[usize]) -> bool {
    let mut dp = vec![vec![0; s + 1]; n];

    // 1行目
    let a1 = a[0];
    for j in 1..=s {
        if j >= a1 && s >= a1 {
            dp[0][j] = a1;
        }
    }

    // 行: カードの枚数0..N, i:i番目のカード
    // 列: 整数の和0..=S, j: そこまでの整数の和j
    for i in 1..n {
        for j in 0..=s {
            // j < a[i] (そこまでの整数の和をカードの値が超える)とき、一つ前のカードのまま
            if j < a[i] {
                dp[i][j] = dp[i - 1][j];
            } else {
                // j >= a[i] (そこまでの整数の和をカードの値が超えない)のとき、
                // 1. 一つ前のカードのまま
                // 2. ーつ前のカードの選択のうち、値jを加算できる最大の値
                // 1,2 を比べた最大値になる
                let v1 = dp[i - 1][j];
                let v2 = dp[i - 1][j - a[i]] + a[i];
                dp[i][j] = v1.max(v2).max(a[i]);
            }
        }

        if dp[i][s] == s {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, 11, &[2, 5, 9]), true);
        assert_eq!(solve(4, 11, &[3, 1, 4, 5]), false);
        assert_eq!(solve(5, 12, &[2, 4, 9, 2, 3]), true);
    }
}
