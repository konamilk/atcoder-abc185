use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        t: i32,
        ab: [(i32, i32); m]
    }

    let mut ans = true;

    let mut rest: i32 = n;
    let mut time = 0;

    for (a,b) in ab.into_iter(){
        rest += time - a;
        time = a;
        if rest < 1{
            ans = false;
            break;
        }
        rest += b - a;
        rest = std::cmp::min(n, rest);
        time = b;
    }

    rest += time - t;
    if rest < 1{
        ans = false;
    }

    println!("{}",if ans {"Yes"} else {"No"})

}
