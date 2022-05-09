use proconio::input;

/// 061 - Stones Game 2
fn main() {
    input! {
        n: u64,
    }

    println!("{}", solve(n));
}

fn solve(n: u64) -> &'static str {
    let mut v = n + 1;

    while v > 1 {
        let x = v % 2;
        if x != 0 {
            return "First";
        }

        v /= 2;
    }

    "Second"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(2), "First");
        assert_eq!(solve(3), "Second");
        assert_eq!(solve(1000000000000000000), "First");
    }
}
