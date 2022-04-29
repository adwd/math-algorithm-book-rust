use proconio::input;

/// 025 - Jiro's Vacation
fn main() {
    input! {
        n: usize,
        a: [u32; n],
        b: [u32; n],
    }

    println!("{}", solve(n, a, b));
}

fn solve(n: usize, a: Vec<u32>, b: Vec<u32>) -> f64 {
    let mut result = 0.0;
    for i in 0..n {
        result += (a[i] as f64) / 3.0 + (b[i] as f64 * 2.0) / 3.0;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(5, vec![3, 1, 4, 1, 5], vec![9, 2, 6, 5, 3]),
            21.333333333333336
        );
    }
}
