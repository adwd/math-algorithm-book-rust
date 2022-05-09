use proconio::input;

/// 054 - Fibonacci Hard (mod 1000000000)
fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

const M: usize = 1_000_000_007;

fn solve(n: usize) -> usize {
    let mut a = Matrix::new();
    a.p[0][0] = 1;
    a.p[0][1] = 1;
    a.p[0][2] = 1;
    a.p[1][0] = 1;
    a.p[2][1] = 1;
    let b = a.power(n - 3);

    (b.p[0][0] * 2 + b.p[0][1] + b.p[0][2]) % M
}

#[derive(Debug, Clone)]
struct Matrix {
    p: [[usize; 3]; 3],
}

impl Matrix {
    fn new() -> Self {
        Self {
            p: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut c = Matrix::new();

        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    c.p[i][j] += self.p[i][k] * other.p[k][j];
                    c.p[i][j] %= M;
                }
            }
        }

        c
    }

    fn power(&self, n: usize) -> Self {
        let mut p = self.clone();
        let mut q = Matrix::new();
        let mut flag = false;

        for i in 0..60 {
            if n & (1 << i) != 0 {
                if !flag {
                    q = p.clone();
                    flag = true;
                } else {
                    q = q.mul(&p);
                }
            }
            p = p.mul(&p);
        }

        q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(4), 4);
        assert_eq!(solve(5), 7);
        assert_eq!(solve(10), 149);
        assert_eq!(solve(876543210987654321), 639479200);
    }
}
