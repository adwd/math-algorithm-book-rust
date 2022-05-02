use proconio::input;

/// 039 - Snowy Days
fn main() {
    input! {
        n: usize,
        q: usize,
        qn: [(usize, usize, i64); q],
    }

    println!("{}", solve(n, q, &qn));
}

fn solve(n: usize, q: usize, qn: &[(usize, usize, i64)]) -> String {
    let mut result = vec![0_i64; n + 1];
    for x in qn {
        let (l, r, k) = *x;
        result[l - 1] += k;
        result[r] -= k;
    }

    let mut answer = String::with_capacity(q);
    for i in 1..n {
        let d = result[i];
        if d > 0 {
            answer.push('<');
        } else if d < 0 {
            answer.push('>');
        } else {
            answer.push('=');
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(5, 3, &[(1, 2, 3), (2, 5, 4), (2, 4, 1)]), "<>=>");
    }
}
