use proconio::input;

fn main() {
    input! {
        l: u64
    }

    let ans = binom_knuth(l-1, std::cmp::min(l-12, 11));

    println!("{}", ans)
}


pub fn binom_knuth(n: u64, k: u64) -> u64 {
    (0..n + 1)
        .rev()
        .zip(1..k + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}