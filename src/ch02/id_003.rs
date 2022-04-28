use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", id_003(n, a));
}

fn id_003(_n: usize, a: Vec<u32>) -> u32 {
    a.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_003(3, vec![1, 2, 3]), 6);
        assert_eq!(id_003(5, vec![0, 10, 20, 30, 40]), 100);
    }
}
