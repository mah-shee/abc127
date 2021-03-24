#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut cards: [usize; n],
        mut cond: [(usize, usize); m],
    }
    cards.sort();
    cond.sort_by(|a, b| a.1.cmp(&b.1));
    let mut a = Vec::<usize>::new();
    for i in (0..m).rev() {
        //
        let mut vec = vec![cond[i].1; cond[i].0];
        a.append(&mut vec);
        if a.len() > n {
            break;
        }
    }
    for i in 0..(std::cmp::min(n, a.len())) {
        if cards[i] > a[i] {
            break;
        } else {
            cards[i] = a[i];
        }
    }
    println!("{}", cards.iter().sum::<usize>());
}
