use proconio::input;

/// 059 - Power of Two
fn main() {
    input! {
        n: u64,
    }

    println!("{}", solve(n));
}

fn solve(n: u64) -> u32 {
    match n % 4 {
        1 => 2,
        2 => 4,
        3 => 8,
        _ => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(10), 4);
    }
}
