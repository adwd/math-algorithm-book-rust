use proconio::input;

fn main() {
    input! {
        n: u64
    }

    println!("{}", solve(n));
}

const M: u64 = 1000000007;

fn solve(n: u64) -> u64 {
    let x = n * (n + 1) / 2 % M;

    x % M * x % M
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(2), 9);
        assert_eq!(solve(869120), 497335961);
    }
}
