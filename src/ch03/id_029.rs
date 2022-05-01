use proconio::input;

/// 029 - Climb Stairs
/// 太郎君は N 段の階段を上ろうとしています。彼は一歩で 1 段か 2 段上ることができます。
/// 0 段目から出発し、N 段目にたどり着くまでの移動方法が何通りあるかを計算してください。
fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn solve(mut n: usize) -> u64 {
    n += 1;
    let mut arr = vec![1_u64; n];
    if n < 3 {
        return 1;
    }

    for i in 2..n {
        arr[i] = arr[(i - 1)] + arr[(i - 2)];
    }

    *arr.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(1), 1);
        assert_eq!(solve(2), 2);
        assert_eq!(solve(3), 3);
        assert_eq!(solve(4), 5);
        assert_eq!(solve(5), 8);
        assert_eq!(solve(6), 13);
        assert_eq!(solve(45), 1836311903);
    }
}
