use proconio::input;

/**
  013 - Divisor Enumeration
  整数Nが与えられます。 Nの約数を列挙してください。
*/
fn main() {
    input! {
        n: u64,
    }

    id_013(n).iter().for_each(|v| println!("{}", v));
}

fn id_013(n: u64) -> Vec<u64> {
    let limit = (n as f64).sqrt().floor() as u64;
    let mut result = vec![];
    for i in 1..=limit {
        if n % i == 0 {
            result.push(i);
            if i != n / i {
                result.push(n / i);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_013(12), vec![1, 12, 2, 6, 3, 4]);
    }
}
