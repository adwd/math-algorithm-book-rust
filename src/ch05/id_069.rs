use proconio::input;

/// 069 - Product Max
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64
    }

    println!("{}", solve(a, b, c, d));
}

fn solve(a: i64, b: i64, c: i64, d: i64) -> i64 {
    (a * c).max(a * d).max(b * c).max(b * d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(1, 2, 1, 1), 2);
    }
}
