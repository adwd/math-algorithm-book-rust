use proconio::input;

/// 060 - Stones Game 1
fn main() {
    input! {
        n: u64,
    }

    println!("{}", solve(n));
}

fn solve(n: u64) -> &'static str {
    if n % 4 == 0 {
        "Second"
    } else {
        "First"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4), "Second");
    }
}
