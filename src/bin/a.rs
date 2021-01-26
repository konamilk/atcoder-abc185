use proconio::input;

fn main() {
    input! {
        a: [i32; 4]
    }

    let ans = a.iter().fold(999999999, |acc, &x| std::cmp::min(acc, x));

    println!("{}", ans)
}
