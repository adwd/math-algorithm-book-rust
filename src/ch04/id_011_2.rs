use proconio::input;

/// 011 - Print Prime Numbers
/// N 以下の素数を、小さい順に出力してください。
/// 制約
/// 2 ≤ N ≤ 3000
/// N は整数
fn main() {
    input! {
        n: usize,
    }

    solve(n).iter().for_each(|v| print!("{} ", v));
}

fn solve(n: usize) -> Vec<usize> {
    let mut prime = vec![true; n + 1];

    let root_n = (n as f64).sqrt().floor() as usize;
    for i in 2..=root_n {
        if prime[i] {
            for j in ((2 * i)..=n).step_by(i) {
                prime[j] = false;
            }
        }
    }

    let mut result = vec![];
    prime.iter().enumerate().skip(2).for_each(|(v, prime)| {
        if *prime {
            result.push(v);
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(10), vec![2, 3, 5, 7]);
    }
}
