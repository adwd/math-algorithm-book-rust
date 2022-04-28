use proconio::input;

/**
  014 - Factorization
  自然数 N を素因数分解するプログラムを作成してください。
  なお、任意の自然数の素因数分解は一意となることが知られています。
*/
fn main() {
    input! {
        n: u64,
    }

    id_014(n).iter().for_each(|v| print!("{} ", v));
}

fn id_014(n: u64) -> Vec<u64> {
    let mut result = vec![];

    let mut solve = |nn: u64| {
        let d = devisor(nn);
        match d {
            Some(x) => {
                result.push(x);
                Some(x)
            }
            None => None,
        }
    };

    let mut v = n;
    loop {
        let x = solve(v);
        if let Some(xx) = x {
            v /= xx;
        } else {
            result.push(v);
            break;
        }
    }

    result
}

fn devisor(n: u64) -> Option<u64> {
    let limit = (n as f64).sqrt().floor() as u64;
    for i in 2..=limit {
        if n % i == 0 {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn devisor_works() {
        assert_eq!(devisor(12), Some(2));
        assert_eq!(devisor(9), Some(3));
        assert_eq!(devisor(4), Some(2));
        assert_eq!(devisor(19), None);
        assert_eq!(devisor(2), None);
        assert_eq!(devisor(1), None);
    }

    #[test]
    fn it_works() {
        assert_eq!(id_014(10), vec![2, 5]);
        assert_eq!(id_014(36), vec![2, 2, 3, 3]);
    }
}
