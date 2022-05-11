use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    println!("{}", solve(a, b));
}

fn solve(a: u64, b: u64) -> String {
    let limit = 10_u64.pow(18);
    let answer = lcm(a, b);

    match answer {
        None => "Large".to_owned(),
        Some(a) => {
            if a > limit {
                "Large".to_owned()
            } else {
                a.to_string()
            }
        }
    }
}

fn lcm(a: u64, b: u64) -> Option<u64> {
    let g = b / gcd(a, b);
    a.checked_mul(g)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4, 6), "12");
        assert_eq!(solve(1000000000000000000, 3), "Large");
        assert_eq!(solve(1000000000000000000, 1), "1000000000000000000");
    }
}
