use proconio::input;

fn main() {
    input! {
        a1: i32,
        a2: i32,
        a3: i32,
    }

    println!("{}", id_002(a1, a2, a3));
}

fn id_002(a1: i32, a2: i32, a3: i32) -> i32 {
    a1 + a2 + a3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(id_002(1, 2, 3), 6);
        assert_eq!(id_002(99, 50, 80), 229);
    }
}
