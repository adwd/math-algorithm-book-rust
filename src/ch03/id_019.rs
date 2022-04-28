use proconio::input;

/**
 * 019 - Choose Cards 1
 * N 枚のカードがあり、左から i 番目（1 ≤ i ≤ N）のカードの色は Ai  です。 Ai =1 のとき赤色、Ai =2 のとき黄色、Ai =3 のとき青色です。
 * 同じ色のカードを 2 枚選ぶ方法は何通りありますか。
 * 1 ≤ Ai ≤ 3
 */
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    println!("{}", solve(a));
}

fn solve(a: Vec<u64>) -> u64 {
    // 3種類の色ごとの枚数を調べ、xC2を計算して足す
    let mut n1 = 0;
    let mut n2 = 0;
    let mut n3 = 0;
    for item in a.iter() {
        match item {
            1 => n1 += 1,
            2 => n2 += 1,
            3 => n3 += 1,
            _ => {}
        }
    }

    c2(n1) + c2(n2) + c2(n3)
}

fn c2(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    (n * (n - 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(vec![1, 3, 2, 1, 1, 2]), 4);
        assert_eq!(solve(vec![1, 1]), 1);
        assert_eq!(solve(vec![1, 1, 2, 2]), 2);
        assert_eq!(solve(vec![1, 1, 2, 2, 3]), 2);
        assert_eq!(solve(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(solve(vec![1, 1, 2, 2, 3, 3, 3]), 5);
        assert_eq!(solve(vec![1, 1, 2, 2, 3, 3, 3, 3]), 8);
        assert_eq!(solve(vec![1, 2]), 0);
        assert_eq!(
            solve(std::iter::repeat(1).take(1000).collect()),
            1000 * 999 / 2
        );
        assert_eq!(
            solve(std::iter::repeat(1).take(500000).collect()),
            500000 * (500000 - 1) / 2
        );
    }
}
