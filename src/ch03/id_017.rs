use proconio::input;

/**
  017 - Least Common Multiple of N Integers
  N 個の正の整数 A1 ,A2 ,…,AN の最小公倍数を求めてください。
*/
fn main() {
    input! {
        _n: usize,
        a: [u64; _n],
    }

    println!("{}", id_017(a));
}

fn id_017(a: Vec<u64>) -> u64 {
    a.iter().fold(1, |acc, x| lcm(acc, *x))
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_017(vec![12, 18, 14]), 252);
        assert_eq!(id_017(vec![120, 156, 180]), 4680);
    }
}
