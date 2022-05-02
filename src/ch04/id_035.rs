use proconio::derive_readable;
use proconio::input;

/// 035 - Two Circles
/// 二次元平面上に、以下の 2 つの円があります。
/// - 1 つ目の円の中心座標は (x1, y1)、半径は r1
/// - 2 つ目の円の中心座標は (x2, y2)、半径は r2
/// さて、2 つの円の位置関係は以下の 5 通りのいずれかです。
/// [1]　一方の円が他方の円を完全に含み、2 つの円は接していない
/// [2]　一方の円が他方の円を完全に含み、2 つの円は接している
/// [3]　2 つの円が互いに交差する
/// [4]　2 つの円の内部に共通部分は存在しないが、2 つの円は接している
/// [5]　2 つの円の内部に共通部分は存在せず、2 つの円は接していない
/// 与えられた 2 つの円に当てはまる位置関係の番号を出力してください。
/// 0 ≤ x1, x2 ,y1, y2 ≤10^6
/// 1 ≤ r1 ,r2 ≤ 10^6
/// 入力はすべて整数
fn main() {
    input! {
        x1: i64,
        y1: i64,
        r1: u64,
        x2: i64,
        y2: i64,
        r2: u64,
    }

    println!("{}", solve(Point::new(x1, y1), r1, Point::new(x2, y2), r2));
}

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

struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn from_points(p1: &Point, p2: &Point) -> Vector {
        Vector {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        }
    }

    fn product(&self, v: &Vector) -> i64 {
        self.x * v.x + self.y * v.y
    }

    fn cross(&self, v: &Vector) -> i64 {
        self.x * v.y - self.y * v.x
    }

    fn len(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

fn solve(p1: Point, r1: u64, p2: Point, r2: u64) -> u16 {
    let r1 = r1 as f64;
    let r2 = r2 as f64;

    // 半径が大きい方の円をa、小さい方の円をbとする
    let ((ap, ar), (bp, br)) = if r1 >= r2 {
        ((p1, r1), (p2, r2))
    } else {
        ((p2, r2), (p1, r1))
    };

    // p1, p2の中心間の距離と、それぞれの円の半径を比較する
    let d = ap.distance(&bp);

    if d == ar + br {
        4
    } else if d > ar + br {
        5
    } else if ar == d + br {
        2
    } else if ar > d + br {
        1
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(Point::new(4, 1), 2, Point::new(1, 5), 3), 4);
    }
}
