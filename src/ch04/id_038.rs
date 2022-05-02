use proconio::input;

/// 038 - How Many Guests?
fn main() {
    input! {
        n: usize,
        q: usize,
        an: [usize; n],
        qn: [(usize, usize); q],
    }

    solve(n, q, &an, &qn).iter().for_each(|v| println!("{}", v));
}

fn solve(n: usize, _q: usize, an: &[usize], qn: &[(usize, usize)]) -> Vec<usize> {
    let mut ps = vec![0; n + 1];
    for (index, v) in an.iter().enumerate() {
        ps[index + 1] = ps[index] + v;
    }

    qn.iter().map(|(l, r)| ps[*r] - ps[*l - 1]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     solve(10, 1, &[8, 6, 9, 1, 2, 1, 10, 100, 1000, 10000], &[(2, 3)]),
        //     vec![15]
        // );
        assert_eq!(
            solve(
                10,
                5,
                &[8, 6, 9, 1, 2, 1, 10, 100, 1000, 10000],
                &[(2, 3), (1, 4), (3, 9), (6, 8), (1, 10)]
            ),
            vec![15, 24, 1123, 111, 11137]
        );
    }
}
