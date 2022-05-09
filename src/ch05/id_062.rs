use proconio::input;

/// 062 - Teleporter
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    println!("{}", solve(n, k, &a));
}

fn solve(n: usize, k: usize, a: &[usize]) -> usize {
    let mut first = vec![-1_i64; n + 1];
    let mut second = vec![-1_i64; n + 1];

    let mut count = 0;
    let mut current = 1;

    loop {
        // First, Second の更新
        if first[current] == -1 {
            first[current] = count;
        } else if second[current] == -1 {
            second[current] = count;
        }

        // K 回の移動後に町 cur にいるかどうかの判定
        if count == k as i64
            || (second[current] != -1
                && (k as i64 - first[current]) % (second[current] - first[current]) == 0)
        {
            return current;
        }

        // 位置の更新
        current = a[current - 1];
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4, 5, &[3, 2, 4, 1]), 4);
        assert_eq!(solve(6, 727202214173249351, &[6, 5, 2, 5, 3, 2]), 2);
    }
}
