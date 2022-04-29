use proconio::input;

/**
 * 022 - Choose Cards 3
 * N 枚のカードがあり、左から i 番目のカードには整数 Aiが書かれています。
 * 和が 100000 となる 2 枚のカードの選び方は何通りあるかを求めるプログラムを作成してください。
 * 2 ≤ N ≤ 200000
 * 1 ≤ Ai ≤ 99999
 * 入力はすべて整数
 */
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", solve(n, a));
}

const TOTAL: usize = 100000;

// カードの整数を全部数えて、出現数を値に持つMapを作る
// 1 -> 50000 まで、足して100000nC2を数えて足していく
fn solve(_n: usize, a: Vec<u32>) -> usize {
    let mut result = 0;

    // counts: indexが Ai, 値が枚数の配列
    let mut counts = vec![0_usize; TOTAL];
    for item in a.iter() {
        counts[*item as usize] += 1;
    }

    for i in 1..=(TOTAL / 2) {
        let c = counts[i];
        let cc = counts[TOTAL - i];
        if c == 0 || cc == 0 {
            continue;
        }
        if i == TOTAL - i {
            result += ncr(c, 2);
        } else {
            result += c * cc;
        }
    }

    result
}

fn ncr(n: usize, r: usize) -> usize {
    fact2(n, r) / fact(r)
}

fn fact(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn fact2(n: usize, limit: usize) -> usize {
    if n == 1 || limit == 0 {
        1
    } else {
        n * fact2(n - 1, limit - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(6, vec![40000, 50000, 20000, 80000, 50000, 30000]), 2);
    }
}
