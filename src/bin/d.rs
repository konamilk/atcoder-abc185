use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        mut a: [i32; m]
    }

    a.push(0);
    a.push(n+1);

    a.sort();

    let mut whites = vec![];

    for i in 0..a.len() - 1{
        whites.push(a[i+1] - a[i] - 1);
    }

    let min = whites
        .clone()
        .into_iter()
        .filter(|&x| x > 0)
        .fold(std::i32::MAX, |acc, x| std::cmp::min(acc, x));

    let ans = whites
        .clone()
        .into_iter()
        .filter(|&x| x > 0)
        .fold(0,|acc, x| acc + x / min + if x % min > 0 {1} else {0});

    println!("{}", ans)
}


