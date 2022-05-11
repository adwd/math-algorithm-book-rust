use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    println!("{}", solve(n));
}

fn solve(n: u64) -> u64 {
    let nn = (n as f64).sqrt().floor() as u64;
    let mut result = 1 << 60;

    for i in 1..=nn {
        if n % i != 0 {
            continue;
        }
        let j = n / i;

        if result > 2 * (i + j) {
            result = 2 * (i + j);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(10), 14);
        assert_eq!(solve(869120), 3732);
    }
}
