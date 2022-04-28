use proconio::input;

fn main() {
    input! {
        _n: usize,
        a: [u32; _n],
    }

    println!("{}", id_005(a));
}

fn id_005(a: Vec<u32>) -> u32 {
    let sum: u32 = a.iter().sum();
    sum % 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_005(vec![1, 2, 3]), 6);
        assert_eq!(id_005(vec![0, 10, 20, 30, 40]), 0);
    }
}
