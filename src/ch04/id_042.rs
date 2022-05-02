use proconio::input;

/// 042 - Sum of Divisors
fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn solve(n: usize) -> usize {
    let mut devisors = vec![0; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            devisors[j] += 1;
        }
    }

    let mut result = 0;
    for i in 1..=n {
        result += i * devisors[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4), 23);
        assert_eq!(solve(100), 26879);
        // assert_eq!(solve(10000000), 838627288460105);
    }
}
