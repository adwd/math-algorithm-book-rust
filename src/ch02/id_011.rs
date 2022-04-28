use proconio::input;

/**
  N 以下の素数を、小さい順に出力してください。
  2 ≤ N≤ 3000
*/
fn main() {
    input! {
        n: u32,
    }

    id_011(n).iter().for_each(|x| print!("{} ", x));
}

fn id_011(n: u32) -> Vec<u32> {
    let mut primes = vec![];
    for i in 2..=n {
        if primes.iter().all(|p| i % p != 0) {
            primes.push(i);
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_011(5), vec![2, 3, 5]);
        assert_eq!(id_011(13), vec![2, 3, 5, 7, 11, 13]);
    }
}
