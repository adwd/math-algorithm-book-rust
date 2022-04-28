use proconio::input;

fn main() {
    input! {
        a1: u32,
        a2: u32,
        a3: u32,
    }

    println!("{}", id_004(a1, a2, a3));
}

fn id_004(a1: u32, a2: u32, a3: u32) -> u32 {
    a1 * a2 * a3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_004(3, 4, 5), 60);
        assert_eq!(id_004(100, 100, 2), 20000);
    }
}
