use proconio::input;
fn main() {
    input! {
        a: usize,b:usize,c:usize
    }
    if a <= b && b <= c || c <= b && b <= a {
        println!("Yes");
    } else {
        println!("No");
    }
}
