use proconio::input;

/// 065 - Bishop
fn main() {
    input! {
        h: usize,
        w: usize,
    }

    println!("{}", solve(h, w));
}

fn solve(h: usize, w: usize) -> usize {
    if h == 1 || w == 1 {
        1
    } else {
        (h * w + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(1, 1), 1);
        assert_eq!(solve(1, 2), 1);
        assert_eq!(solve(4, 5), 10);
        assert_eq!(solve(1000000000, 1000000000), 500000000000000000);
    }
}
