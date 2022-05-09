use proconio::input;

/// 064 - All Zero
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    println!("{}", solve(n, k, &a));
}

fn solve(_n: usize, k: usize, a: &[usize]) -> &'static str {
    let sum = a.iter().sum::<usize>();

    if sum % 2 == k % 2 && sum <= k {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, 3, &[2, 0, 1]), "Yes");
    }
}
