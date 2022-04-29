use proconio::input;

/**
 * 020 - Choose Cards 2
 * N 枚のカードがあり、左から i 番目のカードには整数 Ai が書かれています。
 * カードを 5 枚選ぶ方法のうち、選んだカードに書かれた整数の和がちょうど 1000 となるものは何通りありますか。
 * 5 ≤ N ≤ 100
 * 1 ≤ Ai ≤ 1000
 */
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", solve(n, a));
}

fn solve(n: usize, a: Vec<u32>) -> usize {
    // N枚のカードから5枚を選ぶ組み合わせはNC2
    let mut count = 0;

    for i1 in 0..n {
        for i2 in i1 + 1..n {
            for i3 in i2 + 1..n {
                for i4 in i3 + 1..n {
                    for i5 in i4 + 1..n {
                        if a[i1] + a[i2] + a[i3] + a[i4] + a[i5] == 1000 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, vec![100, 150, 200, 250, 300]), 1);
    }
}
