use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    println!("{}", solve(n, &a));
}

const M: u64 = 1000000007;

fn solve(n: usize, a: &[u64]) -> u64 {
    let mut answer = 0;

    let mut fact = vec![0; 200009];
    fact[0] = 1;

    for i in 1..=200000 {
        fact[i] = (fact[i - 1] * i as u64) % M;
    }

    let ncr = |n: u64, r: u64| -> u64 {
        division(
            fact[n as usize],
            fact[r as usize] * fact[(n - r) as usize] % M,
        )
    };

    for i in 1..=n {
        answer += a[i - 1] * ncr((n - 1) as u64, (i - 1) as u64);
        answer %= M;
    }

    answer
}

fn division(a: u64, b: u64) -> u64 {
    (a * mod_pow(b, M - 2)) % M
}

fn mod_pow(a: u64, b: u64) -> u64 {
    let mut p = a;
    let mut answer = 1;

    for i in 0..30 {
        if b & (1 << i) != 0 {
            answer *= p;
            answer %= M;
        }
        p *= p;
        p %= M;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, &[20, 22, 25, 43, 50]), 480);
    }
}
