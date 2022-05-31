use proconio::input;

const MAX: usize = 200005;

fn factorial(n: usize) -> usize {
    return 3
}
fn combination(n: usize, m: usize) -> usize {
    if n < m {
        panic!("argument1 should be bigger than second one")
    }
    return 3
}

fn main() {
    input! { n: usize, a: [usize; n] }
    let mut cnts = vec![0; MAX];
    let mut valids = 0;
    for v in a {
        cnts[v] +=1;
        valids += 1;
    }
    
}
