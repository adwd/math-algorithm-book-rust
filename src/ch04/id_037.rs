use proconio::derive_readable;
use proconio::input;

/// 037 - Intersection
/// 2 次元平面上に 2 つの線分があります。1 つ目の線分は座標 (x1, y1) と 座標 (x2, y2) を結んでいます。
/// 2 つ目の線分は座標 (x3, y3) と 座標 (x4, y4) を結んでいます。
/// この 2 つの線分が交差するならば Yes を、交差しないならば No を出力してください。
/// ここで、2 つの線分が交差しているとは、両方に共通して含まれる点が存在することを言います。
fn main() {
    input! {
        p1: Point,
        p2: Point,
        p3: Point,
        p4: Point,
    }

    println!("{}", if solve(p1, p2, p3, p4) { "Yes" } else { "No" });
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

fn solve(p1: Point, p2: Point, p3: Point, p4: Point) -> bool {
    // let ta = (cx - dx) * (ay - cy) + (cy - dy) * (cx - ax);
    // let tb = (cx - dx) * (by - cy) + (cy - dy) * (cx - bx);
    // let tc = (ax - bx) * (cy - ay) + (ay - by) * (ax - cx);
    // let td = (ax - bx) * (dy - ay) + (ay - by) * (ax - dx);
    let ta = (p3.x - p4.x) * (p1.y - p3.y) + (p3.y - p4.y) * (p3.x - p1.x);
    let tb = (p3.x - p4.x) * (p2.y - p3.y) + (p3.y - p4.y) * (p3.x - p2.x);
    let tc = (p1.x - p2.x) * (p3.y - p1.y) + (p1.y - p2.y) * (p1.x - p3.x);
    let td = (p1.x - p2.x) * (p4.y - p1.y) + (p1.y - p2.y) * (p1.x - p4.x);

    tc * td < 0 && ta * tb < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                Point::new(1, 1),
                Point::new(2, 2),
                Point::new(1, 2),
                Point::new(2, 1)
            ),
            true
        );
    }
}
