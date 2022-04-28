use proconio::input;

/**
  015 - Greatest Common Divisor
  A と B の最大公約数を求めてください。
*/
fn main() {
    input! {
        a: u64,
        b: u64,
    }

    println!("{}", solve(a, b));
}

fn solve(mut a: u64, mut b: u64) -> u64 {
    loop {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(33, 88), 11);
        assert_eq!(solve(123, 777), 3);
    }
}
