mod lib;
fn main() {
    let v_orig = vec![1,5,3,7,4,78,8,5,6,63,8,566344,5,77,42,43647,43,6,785,68,67,534,5,765,746,34,425,4567,456,5,4,5];
    let mut v = v_orig.clone();
    lib::merge_sort(&mut v, 4);
    for k in 2..5{
        let mut cpy = v_orig.clone();
        lib::merge_sort(&mut cpy, k);
        assert_eq!(v, cpy);
    }
    println!(
        "sorted array: {:?}", v
    );

    println!("Hello, world!");
}
