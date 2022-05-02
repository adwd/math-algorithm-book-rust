use proconio::input;

/// 041 - Convenience Store 2
/// あるコンビニは時刻 0 に開店し、時刻 T に閉店します。このコンビニには N 人の従業員が働いており、
/// i 番目 (1 ≤ i ≤ N) の従業員は時刻 Li に出勤し、時刻 Ri に退勤します。
/// t=0,1,2,…,T−1 それぞれについて、時刻 t+0.5 にコンビニにいる従業員の数を出力するプログラムを作成してください。
fn main() {
    input! {
        t: usize,
        n: usize,
        a: [(usize, usize); n],
    }

    solve(t, n, &a).iter().for_each(|v| println!("{}", v));
}

fn solve(t: usize, _n: usize, a: &[(usize, usize)]) -> Vec<i32> {
    let mut ds = vec![0_i32; t + 1];
    for (l, r) in a {
        ds[*l] += 1;
        ds[*r] -= 1;
    }

    let mut result = vec![0; t];
    result[0] = ds[0];
    for i in 1..t {
        result[i] = result[i - 1] + ds[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                10,
                7,
                &[(0, 3), (2, 4), (1, 3), (0, 3), (5, 6), (5, 6), (5, 6)]
            ),
            vec![2, 3, 4, 1, 0, 3, 0, 0, 0, 0]
        );

        assert_eq!(
            solve(
                10,
                8,
                &[
                    (0, 3),
                    (2, 4),
                    (1, 3),
                    (0, 3),
                    (5, 6),
                    (5, 6),
                    (5, 6),
                    (8, 10)
                ]
            ),
            vec![2, 3, 4, 1, 0, 3, 0, 0, 1, 1]
        );
    }
}
