use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
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

/// 033 - Distance
/// 2 次元平面上に点 A,B,C があります。点 A の座標は (ax ,ay)、点 B の座標は (bx,by) 、点 C の座標は (cx,cy) です。
/// 点 A と線分 BC 上の点の最短距離を求めてください。
fn main() {
    input! {
        a: Point,
        b: Point,
        c: Point,
    }

    println!("{}", solve(a, b, c));
}

fn solve(a: Point, b: Point, c: Point) -> f64 {
    let ba = Vector::from_points(&b, &a);
    let bc = Vector::from_points(&b, &c);
    let ca = Vector::from_points(&c, &a);
    let cb = Vector::from_points(&c, &b);

    // BA * BC、CA * CBが負ならそれぞれ|BA|,|CA|が答え
    // そうでないなら三角形の高さを求める
    let pattern = if ba.product(&bc) < 0 {
        1
    } else if ca.product(&cb) < 0 {
        3
    } else {
        2
    };

    match pattern {
        1 => ba.len(),
        3 => ca.len(),
        _ => {
            let s = ba.cross(&bc).abs() as f64;
            s / bc.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                Point { x: 0, y: 5 },
                Point { x: 1, y: 1 },
                Point { x: 3, y: 0 }
            ),
            4.123105625617661
        );
        assert_eq!(
            solve(
                Point { x: 3, y: 4 },
                Point { x: 0, y: 0 },
                Point { x: 5, y: 0 }
            ),
            4.0
        );
        assert_eq!(
            solve(
                Point { x: -3, y: 4 },
                Point { x: 0, y: 0 },
                Point { x: 5, y: 0 }
            ),
            5.0
        );
    }
}
