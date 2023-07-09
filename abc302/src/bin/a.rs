fn main() {
    proconio::input! { a: usize, b: usize }
    let f = (a + b - 1) / b;
    println!("{}", f);
}
