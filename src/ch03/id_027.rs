use proconio::input;

/// 027 - Sorting
/// 長さ N の配列 [A1, A2, ⋯, AN] が与えられます。
/// 書籍に記されている「マージソートを行う未完成のプログラム」を元に、配列を昇順に並び替えるプログラムを作成してください。
/// 制約
/// smallと名前がついているテストケースを正答することで、満点の 50% を得ることができます。 smallのテストケースは以下の制約を満たします。
/// 2 ≤ N ≤ 2000
/// 1 ≤ Ai ≤ 10^9
/// 入力はすべて整数
/// さらに以下の制約を満たすテストケースを正答することで、満点を得ることができます。
/// 2 ≤ N ≤ 200000
/// 1 ≤ Ai ≤ 10^9
/// 入力はすべて整数
fn main() {
    input! {
        n: usize,
        a: [u64; n as usize],
    }

    // println!("{}", solve(n, a));
    solve(n, a).iter().for_each(|v| print!("{} ", v));
}

fn solve(n: usize, mut a: Vec<u64>) -> Vec<u64> {
    let mut sorted = vec![0; n as usize];
    merge_sort(0, n, &mut a, &mut sorted);
    sorted
}

fn merge_sort(l: usize, r: usize, arr: &mut [u64], sorted: &mut [u64]) {
    if r - l == 1 {
        return;
    }

    let m = (l + r) / 2;
    merge_sort(l, m, arr, sorted);
    merge_sort(m, r, arr, sorted);

    let mut c1 = l;
    let mut c2 = m;
    let mut count = 0;
    while c1 != m || c2 != r {
        if c1 == m {
            sorted[count] = arr[c2];
            c2 += 1;
        } else if c2 == r || arr[c1] < arr[c2] {
            sorted[count] = arr[c1];
            c1 += 1;
        } else {
            sorted[count] = arr[c2];
            c2 += 1;
        }
        count += 1;
    }

    arr[l..(count + l)].copy_from_slice(&sorted[..count]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, vec![658, 299, 47]), vec![47, 299, 658]);
        assert_eq!(
            solve(10, vec![658, 299, 47, 507, 122, 969, 449, 68, 513, 800]),
            vec![47, 68, 122, 299, 449, 507, 513, 658, 800, 969]
        );
    }
}
