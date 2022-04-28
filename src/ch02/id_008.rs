use proconio::input;

/**
  赤・青のカードが各 1 枚ずつあり、あなたはそれぞれのカードに 1 以上 N 以下の整数を 1 つ書き込みます。
  カードに書かれた整数の合計が S 以下となる書き方は、いくつありますか？
*/
fn main() {
    input! {
        n: u32,
        s: u32,
    }

    println!("{}", id_008(n, s));
}

fn id_008(n: u32, s: u32) -> u32 {
    let mut count = 0;
    for red in 1..=n {
        for blue in 1..=n {
            if red + blue <= s {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_008(3, 4), 6);
        assert_eq!(id_008(869, 120), 7140);
    }
}
