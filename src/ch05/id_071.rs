use proconio::derive_readable;
use proconio::input;

/// 070 - Axis-Parallel Rectangle
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
    a: i64,
    b: i64,
    c: i64,
}

impl Edge {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self { a, b, c }
    }

    fn from_array(arr: &[(i64, i64, i64)]) -> Vec<Self> {
        arr.iter().map(|&(a, b, c)| Self::new(a, b, c)).collect()
    }
}

fn solve(_n: usize, p: &[Edge]) -> f64 {
    // すべてのp内の2つの組み合わせにおいて、その交点を求めて、その点がp内にあるかどうかを判定する
    let mut result = -1e62;

    for i in p {
        for j in p {
            if i.a == j.a {
                continue;
            }
            let cross = (
                (i.c * j.b - j.c * i.b) as f64 / (i.a * j.b - j.a * i.b) as f64,
                (i.c * j.a - j.c * i.a) as f64 / (i.b * j.a - j.b * i.a) as f64,
            );
            if p.iter()
                .all(|v| v.a as f64 * cross.0 + v.b as f64 * cross.1 <= v.c as f64)
            {
                result = f64::max(result, cross.0 + cross.1);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = Edge::from_array(&[(1, 3, 3), (3, 1, 3)]);
        assert_eq!(solve(2, &edges), 1.5);
    }
}
