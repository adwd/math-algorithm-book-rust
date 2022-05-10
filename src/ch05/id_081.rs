use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn solve(n: usize) -> usize {
    let bills = [10000, 5000, 1000];
    let mut answer = 0;
    let mut state = n;

    for b in bills.iter() {
        while b <= &state {
            state -= b;
            answer += 1;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(29000), 7);
    }
}
