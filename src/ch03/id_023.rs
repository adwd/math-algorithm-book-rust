use proconio::input;

/**
 * 023 - Dice Expectation
 */
fn main() {
    input! {
        n: usize,
        b: [u64; n],
        r: [u64; n],
    }

    println!("{}", solve(n, b, r));
}

fn solve(n: usize, b: Vec<u64>, r: Vec<u64>) -> f64 {
    let blue = b.iter().sum::<u64>();
    let red = r.iter().sum::<u64>();

    ((blue + red) as f64) / (n as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, vec![1, 2, 3], vec![10, 20, 30]), 22.0);
    }
}
