use proconio::input;

/**
 * 018 - Convenience Store 1
 * コンビニには N 個の品物が売られており、i 番目（1 ≤ i ≤ N）の商品の値段は Ai円です。
 * 異なる 2 つの品物を買う方法のうち、合計金額が 500 円となるものは何通りありますか。
 * Ai は 100,200,300,400 のいずれか
 */
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", solve(n, a));
}

fn solve(n: usize, a: Vec<u32>) -> u32 {
    // 100, 200, 300, 400円の商品の数をそれぞれ数え、個数を(s,t,u,v)とすると
    // sv + tu が組み合わせの数となる
    let mut s = 0;
    let mut t = 0;
    let mut u = 0;
    let mut v = 0;
    for item in a.iter() {
        match item {
            100 => s += 1,
            200 => t += 1,
            300 => u += 1,
            400 => v += 1,
            _ => unreachable!(),
        }
    }

    (s * v) + (t * u)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, vec![100, 300, 400, 400, 200]), 3);
    }
}
