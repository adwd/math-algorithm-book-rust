use proconio::input;

fn main() {
    input! {
        n: u64,
        x: u64
    }

    println!("{}", solve(n, x));
}

fn solve(n: u64, x: u64) -> u64 {
    let mut count = 0;
    for a in 1..=n {
        for b in a + 1..=n {
            for c in b + 1..=n {
                if a + b + c == x {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, 9), 2);
    }
}
