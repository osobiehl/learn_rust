mod lib;
use std::time::SystemTime;
use rand::prelude::*;
fn compute_and_run<T>(arr: &mut [T], f: fn(&mut [T])->() ) -> u128 where
T: PartialOrd + Copy + std::fmt::Debug{
    let now = SystemTime::now();
    f(arr);
    let stop = now.elapsed().unwrap().as_micros();
    return stop;
}
const N: u128 = 100;
const SIZE: usize = 10000;
fn main() {
    for s in (25..SIZE).step_by(SIZE / 10){

        let fns = [lib::heap_sort::<i32>, lib::heap_sort_bottomup::<i32>, lib::bubble_sort::<i32>];
        let mut avg: [u128;3] = [0,0,0];
        for _ in  0..N{
            let v: Vec<i32> = (0..s).map(|_| rand::thread_rng().gen_range(0..=100)).collect();
            
            for i in 0..fns.len(){
    
                let mut v_temp = v.clone();
                avg[i] += compute_and_run(&mut v_temp, fns[i]);
            }
    
        }
        for i in 0..avg.len(){
            avg[i] /= N;
        }
        println!("results for size: {}\n \
    heap: {}  us\n\
    heap_bottomup: {} us\n \
    bubble: {} us\n", s, avg[0], avg[1], avg[2]);
    }

    }


