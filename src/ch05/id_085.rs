use proconio::input;

fn main() {
    input! {
        n: u64,
        x: u64,
        y: u64
    }

    println!("{}", solve(n, x, y));
}

fn solve(n: u64, x: u64, y: u64) -> &'static str {
    // a <= b <= c <= d としても一般性を失わない
    for a in 1..=n {
        for b in a..=n {
            for c in b..=n {
                for d in c..=n {
                    if a + b + c + d == x && a * b * c * d == y {
                        return "Yes";
                    }
                }
            }
        }
    }
    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(6, 11, 30), "Yes");
    }
}
