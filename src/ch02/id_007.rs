use proconio::input;

/**
  問題文
  N 以下の正の整数の中で、X の倍数または Y の倍数であるものの個数はいくつありますか？
*/
fn main() {
    input! {
        n: u32,
        x: u32,
        y: u32,
    }

    println!("{}", id_007(n, x, y));
}

fn id_007(n: u32, x: u32, y: u32) -> u32 {
    let mut count = 0;
    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_007(15, 3, 5), 7);
        assert_eq!(id_007(1000000, 11, 13), 160839);
    }
}
