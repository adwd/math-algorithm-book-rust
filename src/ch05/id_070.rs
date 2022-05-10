use proconio::derive_readable;
use proconio::input;

/// 070 - Axis-Parallel Rectangle
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Edge; n]
    }

    println!("{}", solve(n, k, &p));
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

fn solve(n: usize, k: usize, p: &[Edge]) -> i64 {
    let mut answer = 1 << 62;

    for i1 in 0..n {
        for i2 in 0..n {
            for i3 in 0..n {
                for i4 in 0..n {
                    let cl = p[i1].x;
                    let cr = p[i2].x;
                    let dl = p[i3].y;
                    let dr = p[i4].y;
                    if check_points(cl, cr, dl, dr, p) >= k {
                        let area = (cr - cl) * (dr - dl);
                        answer = answer.min(area);
                    }
                }
            }
        }
    }

    answer
}

fn check_points(lx: i64, rx: i64, ly: i64, ry: i64, pp: &[Edge]) -> usize {
    let mut count = 0;

    for p in pp {
        if lx <= p.x && p.x <= rx && ly <= p.y && p.y <= ry {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = Edge::from_array(&[(1, 4), (3, 3), (6, 2), (8, 1)]);
        assert_eq!(solve(4, 4, &edges), 21);

        let edges = Edge::from_array(&[(0, 0), (1, 1), (2, 2), (3, 3)]);
        assert_eq!(solve(4, 2, &edges), 1);

        let edges = Edge::from_array(&[
            (-1000000000, -1000000000),
            (1000000000, 1000000000),
            (-999999999, 999999999),
            (999999999, -999999999),
        ]);
        assert_eq!(solve(4, 3, &edges), 3999999996000000001);
    }
}
