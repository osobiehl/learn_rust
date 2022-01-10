mod lib;
use std::time::SystemTime;
fn compute_and_run<T>(arr: &mut [T], f: fn(&mut [T])->() ) -> u128 where
T: PartialOrd + Copy + std::fmt::Debug{
    let now = SystemTime::now();
    f(arr);
    let stop = now.elapsed().unwrap().as_micros();
    return stop;
}
const N: i32 = 100;
const SIZE: usize = 100000;
fn main() {
    let fns = [lib::heap_sort::<i32>, lib::heap_sort_bottomup::<i32>, lib::bubble_sort::<i32>];
    let mut avg: [u128;3] = [0,0,0];
    for _ in  0..N{
        let v: Vec<i32> = (0..SIZE).map(|_| rand::thread_rng().gen_range(0..=100)).collect();
        
        for i in 0..fns.len(){

            let mut v_temp = v.clone();
            avg[i] += compute_and_run(&mut v_temp, fns[i]);
        }
        avg3 += compute_and_run_3(&mut v.clone())
    }
    for i in 0..avg.len(){
        avg[i] /= N;
    }
    avg3 /= N;
    println!("results:\n \
default: {}  us\n\
Hoare: {} us\n \
MedianOfThree: {} us\n
average of 3: {} us", avg[0], avg[1], avg[2], avg3);
}

