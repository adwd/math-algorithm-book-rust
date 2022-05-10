use proconio::derive_readable;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [Edge; n]
    }

    println!("{}", solve(n, &p));
}

#[derive_readable]
#[derive(Debug)]
struct Edge {
    x: i64,
    y: i64,
}

impl Edge {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn from_array(arr: &[(i64, i64)]) -> Vec<Self> {
        arr.iter().map(|&(x, y)| Self::new(x, y)).collect()
    }
}

fn solve(n: usize, p: &[Edge]) -> i64 {
    // sigmaをXとYで分ける
    // X, Yそれぞれソートすれば絶対値ではなく引き算にできる

    let mut sorted_x = p.iter().map(|v| v.x).collect::<Vec<i64>>();
    sorted_x.sort_unstable();
    let mut sorted_y = p.iter().map(|v| v.y).collect::<Vec<i64>>();
    sorted_y.sort_unstable();

    let mut sum = 0;
    let n = n as i64;
    for i in 1..=n {
        sum += sorted_x[i as usize - 1] * (-n + 2 * i - 1);
        sum += sorted_y[i as usize - 1] * (-n + 2 * i - 1);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = Edge::from_array(&[(1, 2), (10, 20)]);
        assert_eq!(solve(2, &edges), 27);
    }
}
