use proconio::input;

/// 067 - Cross Sum
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    print_answer(&solve(h, w, &a));
}

fn solve(h: usize, w: usize, a: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut sum_row = vec![0; h];
    let mut sum_col = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            sum_row[i] += a[i][j];
            sum_col[j] += a[i][j];
        }
    }

    let mut answer = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            answer[i][j] = sum_row[i] + sum_col[j] - a[i][j];
        }
    }

    answer
}

fn print_answer(a: &[Vec<usize>]) {
    for row in a {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
