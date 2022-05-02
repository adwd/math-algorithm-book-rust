use proconio::input;

/// 036 - : (Colon)
/// 時針と分針の長さがそれぞれ A センチメートル、B センチメートルであるアナログ時計を考えます。
/// 時針と分針それぞれの片方の端点は同じ定点に固定されており、この点を中心としてそれぞれの針は一定の角速度で時計回りに回転します。
/// 時針は 12 時間で、分針は 1 時間で 1 周します。
/// 0 時ちょうどに時針と分針は重なっていました。ちょうど H 時 M 分になったとき、
/// 2 本の針の固定されていない方の端点は何センチメートル離れているでしょうか。
/// 入力はすべて整数
/// 1 ≤ A, B ≤ 1000
/// 0 ≤ H ≤ 11
/// 0 ≤ M ≤ 59
fn main() {
    input! {
        a: u32,
        b: u32,
        h: u32,
        m: u32,
    }

    println!("{}", solve(a, b, h, m));
}

fn solve(a: u32, b: u32, h: u32, m: u32) -> f64 {
    use std::f64::consts::PI;

    let ma = m as f64 / 60.0 * 2.0 * PI;
    let ha = h as f64 / 12.0 * 2.0 * PI + ma / 12.0;

    let ph = (a as f64 * ha.sin(), a as f64 * ha.cos());
    let pm = (b as f64 * ma.sin(), b as f64 * ma.cos());

    let v = (ph.0 - pm.0, ph.1 - pm.1);

    (v.0 * v.0 + v.1 * v.1).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, 4, 9, 0), 5.000000000000001);
    }
}
