use proconio::input;

/**
 * 021 - Combination Easy
 * 整数 n,r が与えられます。
 * nCr を出力するプログラムを作成してください。
 * 1 ≤ r ≤ n ≤ 20
 */
fn main() {
    input! {
        n: u64,
        r: u64,
    }

    println!("{}", solve(n, r));
}

fn solve(n: u64, r: u64) -> u64 {
    fact2(n, r) / fact(r)
}

fn fact(n: u64) -> u64 {
    if n == 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn fact2(n: u64, limit: u64) -> u64 {
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
    fn fact_works() {
        assert_eq!(fact(5), 120);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(1), 1);
    }
    #[test]
    fn fact2_works() {
        assert_eq!(fact2(10, 5), 10 * 9 * 8 * 7 * 6);
        assert_eq!(fact2(20, 2), 380);
        assert_eq!(fact2(5, 2), 20);
        assert_eq!(fact2(2, 1), 2);
        assert_eq!(fact2(1, 0), 1);
    }

    #[test]
    fn it_works() {
        assert_eq!(solve(6, 2), 15);
    }
}
