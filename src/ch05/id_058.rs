use proconio::input;

/// 058 - Move on Squares 1
fn main() {
    input! {
        n: i64,
        x: i64,
        y: i64,
    }

    println!("{}", solve(n, x, y));
}

fn solve(n: i64, x: i64, y: i64) -> &'static str {
    let sum = x.abs() + y.abs();

    if sum <= n && sum % 2 == n % 2 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(10, 2, 2), "Yes");
        assert_eq!(solve(9, 3, 1), "No");
    }
}
