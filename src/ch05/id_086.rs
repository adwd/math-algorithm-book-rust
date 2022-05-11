use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }

    println!("{}", solve(n, &s));
}

fn solve(_n: usize, s: &str) -> &'static str {
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => return "No",
        }
        if depth < 0 {
            return "No";
        }
    }

    if depth == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(8, "(()())()"), "Yes");
    }
}
