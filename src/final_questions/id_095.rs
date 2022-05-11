use proconio::{derive_readable, input};

fn main() {
    input! {
        n: usize,
        s:[Student; n],
        q: usize,
        lr: [Section; q],
    }

    solve(n, &s, q, &lr)
        .iter()
        .for_each(|v| println!("{} {}", v.0, v.1));
}

#[derive_readable]
#[derive(Debug)]
struct Student {
    class: u8,
    score: usize,
}

impl Student {
    fn new(class: u8, score: usize) -> Self {
        Self { class, score }
    }

    fn to_score(&self) -> (usize, usize) {
        if self.class == 1 {
            (self.score, 0)
        } else {
            (0, self.score)
        }
    }

    fn from_array(arr: &[(u8, usize)]) -> Vec<Self> {
        arr.iter().map(|&(x, y)| Self::new(x, y)).collect()
    }
}

#[derive_readable]
#[derive(Debug)]
struct Section {
    from: usize,
    end: usize,
}

impl Section {
    fn new(from: usize, end: usize) -> Self {
        Self { from, end }
    }

    fn from_array(arr: &[(usize, usize)]) -> Vec<Self> {
        arr.iter().map(|&(x, y)| Self::new(x, y)).collect()
    }
}

fn solve(n: usize, s: &[Student], _q: usize, lr: &[Section]) -> Vec<(usize, usize)> {
    // sum[i]: 学生番号i番目までの点数の合計
    let mut sum = vec![(0, 0); n + 1];
    if s[0].class == 1 {
        sum[1] = (s[0].score, 0);
    } else {
        sum[1] = (0, s[0].score);
    }

    for i in 2..=n {
        let score = s[i - 1].to_score();
        sum[i] = (sum[i - 1].0 + score.0, sum[i - 1].1 + score.1);
    }

    lr.iter()
        .map(|s| {
            (
                sum[s.end].0 - sum[s.from - 1].0,
                sum[s.end].1 - sum[s.from - 1].1,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            solve(
                7,
                &Student::from_array(&[
                    (1, 72),
                    (2, 78),
                    (2, 94),
                    (1, 23),
                    (2, 89),
                    (1, 40),
                    (1, 75)
                ]),
                1,
                &Section::from_array(&[(1, 3)])
            ),
            vec![(72, 172)]
        );

        assert_eq!(
            solve(
                7,
                &Student::from_array(&[
                    (1, 72),
                    (2, 78),
                    (2, 94),
                    (1, 23),
                    (2, 89),
                    (1, 40),
                    (1, 75)
                ]),
                1,
                &Section::from_array(&[(2, 6)])
            ),
            vec![(63, 261)]
        );
    }
}
