use proconio::input;

fn main() {
    input!{
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }

    a.sort();
    b.sort();

    println!("{}",
      a.iter().zip(b.iter()).map(|(x, y)| isize::abs(x - y)).sum::<isize>()
    );
}
