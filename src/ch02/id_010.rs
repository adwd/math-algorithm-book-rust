use proconio::input;

/**
  N! の値を求めてください。
 1 ≤ N ≤ 20
*/
fn main() {
    input! {
        n: u32,
    }

    println!("{}", id_010(n));
}

fn id_010(n: u32) -> u32 {
    (1..n + 1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_010(5), 120);
        assert_eq!(id_010(1), 1);
    }
}
