mod lib;

fn main() {

    let mut v = vec![5,8,67,9,3,5,8,1,3,8,5,3];
    println!("Hello, world!");
    lib::selection_sort(&mut v);
    println!("{:?}", &v);
}
