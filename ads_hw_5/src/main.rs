mod lib;
fn main() {
    const n: usize = 15;
    for i in 0..n{
        println!("matrix: {}", lib::fib_matrix(i));
        println!("bottom up: {}", lib::fib_bottomup(i));
        println!("naive: {}", lib::fibonacci_naive(i));
    }

}
