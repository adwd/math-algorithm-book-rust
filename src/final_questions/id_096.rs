use proconio::input;

fn main() {
    input! {
        n: usize,
        t:[usize; n]
    }

    println!("{}", solve(n, &t));
}

fn solve(n: usize, t: &[usize]) -> usize {
    let mut dp = vec![vec![false; 100001]; n + 1];
    let mut sum_t = 0;
    for i in 1..=n {
        sum_t += t[i - 1];
    }
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=sum_t {
            if j < t[i - 1] {
                if dp[i - 1][j] {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            }
            if j >= t[i - 1] {
                if dp[i - 1][j] || dp[i - 1][j - t[i - 1]] {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            }
        }
    }

    let mut answer = 1 << 60;
    for i in 0..=sum_t {
        if dp[n][i] {
            let time = i.max(sum_t - i);
            answer = answer.min(time);
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, &[8, 3, 7, 2, 5]), 13);
        assert_eq!(solve(2, &[1000, 1]), 1000);
    }
}
