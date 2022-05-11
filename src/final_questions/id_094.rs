use proconio::input;

fn main() {
    input! {
        n: u64,
        b:[u64; n - 1]
    }

    println!("{}", solve(n, &b));
}

fn solve(n: u64, b: &[u64]) -> u64 {
    let mut result = b[0] + b[b.len() - 1];
    for i in 0..(n - 2) as usize {
        result += b[i].min(b[i + 1]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, &[2, 5]), 9);
        assert_eq!(solve(6, &[0, 153, 10, 10, 23]), 53);
    }
}
