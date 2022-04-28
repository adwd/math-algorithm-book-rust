use proconio::input;

/**
  N 枚のカードが横一列に並べられています。左から i 番目 (1 ≤ i ≤ N) のカードには整数 A_i が書かれています。
  カードの中からいくつかを選んで、合計がちょうど S となるようにする方法はありますか。
*/
fn main() {
    input! {
        n: u32,
        s: u32,
        a: [u32; n],
    }

    println!("{}", if id_009(n, s, a) { "Yes" } else { "No" });
}

// ビット全探索
// O(N2^N)
fn id_009(n: u32, s: u32, a: Vec<u32>) -> bool {
    let pattern = 2_u32.pow(n);

    for p in 0..pattern {
        let selections = selection(n, p);
        let mut sum = 0;
        for i in 0..a.len() {
            if selections[i] {
                sum += a[i];
            }
        }
        if sum == s {
            return true;
        }
    }

    false
}

fn selection(n: u32, pattern: u32) -> Vec<bool> {
    let mut result = vec![false; n as usize];
    for i in 0..n {
        if pattern & (1 << i) != 0 {
            result[i as usize] = true;
        }
    }

    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_works() {
        assert_eq!(selection(3, 2), vec![false, true, false]);
        assert_eq!(selection(3, 7), vec![true, true, true]);
        assert_eq!(selection(4, 11), vec![true, false, true, true]);
    }

    #[test]
    fn it_works() {
        assert!(id_009(3, 11, vec![2, 5, 9]));
        assert!(!id_009(4, 11, vec![3, 1, 4, 5]));
    }
}
