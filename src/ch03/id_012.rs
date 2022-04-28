use proconio::input;

/**
  012 - Primality Test
  N が素数であるかどうかを判定してください。
*/
fn main() {
    input! {
        n: u64,
    }

    println!("{}", if id_012(n) { "Yes" } else { "No" });
}

fn id_012(n: u64) -> bool {
    let limit = (n as f64).sqrt().floor() as u64;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(!id_012(77));
        assert!(id_012(472249589291));
    }
}
