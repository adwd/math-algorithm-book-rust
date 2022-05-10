use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n]
    }

    println!("{}", solve(n, &lr));
}

fn solve(_n: usize, lr: &[(usize, usize)]) -> usize {
    let mut sorted_times = lr.iter().collect::<Vec<&(usize, usize)>>();
    sorted_times.sort_by(|v1, v2| v1.1.cmp(&v2.1));

    let mut current_endtime = sorted_times[0].1;
    let mut count = 1;

    for t in sorted_times.iter().skip(1) {
        if t.0 < current_endtime {
            continue;
        }

        count += 1;
        current_endtime = t.1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(3, &[(123, 86399), (1, 86400), (86399, 86400)]), 2);
    }
}
