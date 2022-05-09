use proconio::input;

/// 062 - Teleporter
fn main() {
    input! {
        n: usize
    }

    println!("{}", solve(n));
}

fn solve(n: usize) -> &'static str {
    if n % 2 == 0 {
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
        assert_eq!(solve(3), "No");
    }
}
