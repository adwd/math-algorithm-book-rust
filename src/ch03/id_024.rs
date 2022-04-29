use proconio::input;

/**
 * 024 - Answer Exam Randomly
 */
fn main() {
    input! {
        n: usize,
        pq: [(u32, u32); n],
    }

    println!("{}", solve(n, pq));
}

fn solve(_n: usize, pq: Vec<(u32, u32)>) -> f64 {
    pq.iter().map(|item| item.1 as f64 / item.0 as f64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(2, vec![(2, 50), (4, 100)]), 50_f64);
    }
}
