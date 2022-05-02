use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn distance(&self, p: &Point) -> f64 {
        (((self.x - p.x).pow(2) + (self.y - p.y).pow(2)) as f64).sqrt()
    }
}

/// 034 - Nearest Points
/// 2 次元平面上に N 個の点があり、i 番目の点 (1 ≤ i ≤ N) の座標は (xi, yi) です。
/// 最も近い 2 つの点の距離を求めてください。
fn main() {
    input! {
        n: usize,
        p: [Point; n],
    }

    println!("{}", solve(n, p));
}

fn solve(n: usize, a: Vec<Point>) -> f64 {
    let mut result = std::f64::MAX;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            result = result.min(a[i].distance(&a[j]));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                4,
                vec![
                    Point::new(0, 1),
                    Point::new(2, 0),
                    Point::new(2, 3),
                    Point::new(3, 1),
                ]
            ),
            std::f64::consts::SQRT_2
        );
    }
}
