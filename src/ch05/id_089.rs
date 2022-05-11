use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }

    println!("{}", solve(a, b, c));
}

fn solve(a: u64, b: u64, c: u64) -> &'static str {
    // a < c^b を調べれば良い
    // そのままやるとオーバーフローしてしまうので、累乗の途中計算で超えたらやめるようにする
    // c = 1の場合は何乗しても変わらない
    if c == 1 {
        return "No";
    }

    let mut acc = 1;
    for _ in 0..b {
        acc *= c;
        if a < acc {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4, 3, 2), "Yes");
        assert_eq!(solve(869120, 5, 15), "No");
        assert_eq!(
            solve(100000000000000000, 1000000000000000000, 1000000000000000000),
            "Yes"
        );
    }
}
