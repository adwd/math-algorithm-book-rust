use proconio::input;

/// 026 - Coin Gacha
fn main() {
    input! {
        n: u64,
    }

    println!("{}", solve(n));
}

fn solve(n: u64) -> f64 {
    let mut sum = 1.0;
    for i in 1..n {
        sum += 1.0 / (i + 1) as f64;
    }

    sum * (n as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5), 11.416666666666666);
    }
}
