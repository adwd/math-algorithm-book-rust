use proconio::input;

/// 072 - Max GCD 2
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", solve(a, b));
}

fn solve(a: usize, b: usize) -> usize {
    let mut answer = 0;

    for i in 1..=b {
        if shou(a, b, i) {
            answer = i;
        }
    }

    answer
}

fn shou(a: usize, b: usize, t: usize) -> bool {
    let cl = (a + t - 1) / t;
    let cr = b / t;
    cr > cl
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(2, 4), 2);
    }
}
