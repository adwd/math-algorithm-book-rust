use proconio::input;

/// 032 - Binary Search
/// 長さ N の配列 A=[A1, ⋯, AN] と 1 個の質問（クエリ）が与えられます。 質問の内容は以下の通りです。
/// 質問: 要素 X は配列 A の中にありますか？
/// 与えられた質問について、答えを出力するプログラムを作成してください。
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    // println!("{}", solve(n, x, &a));
    println!("{}", if solve(n, x, &a) { "Yes" } else { "No" });
}

fn solve(n: usize, x: usize, a: &[usize]) -> bool {
    a.iter().any(|v| *v == x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(7, 3, &[1, 2, 3, 4, 5, 6, 7]), true);
    }
}
