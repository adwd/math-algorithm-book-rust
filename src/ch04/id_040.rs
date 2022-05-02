use proconio::input;

/// 040 - Travel
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        m: usize,
        b: [usize; m],
    }

    println!("{}", solve(n, &a, m, &b));
}

fn solve(n: usize, a: &[usize], m: usize, b: &[usize]) -> usize {
    let mut ds = vec![0; n];
    for i in 1..n {
        ds[i] = ds[i - 1] + a[i - 1];
    }

    let mut result = 0;
    for i in 1..m {
        result += (ds[b[i - 1] - 1] as i64 - ds[b[i] - 1] as i64).abs();
    }

    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4, &[8, 6, 9], 6, &[2, 1, 3, 2, 3, 4]), 43);
        assert_eq!(solve(4, &[8, 6, 9], 2, &[1, 4]), 23);
        assert_eq!(solve(4, &[8, 6, 9], 3, &[1, 4, 1]), 46);
        assert_eq!(solve(4, &[8, 6, 9], 4, &[1, 2, 3, 4]), 23);
    }
}
