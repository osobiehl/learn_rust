mod lib;
use std::time::{Duration, SystemTime};
use std::vec;
use rand::prelude::*;
use lib::*;

fn compute_and_run<T: Copy + PartialOrd>(arr: &mut [T], f: fn(&mut [T])->usize)->u128{
        let now = SystemTime::now();
        QuickSort(arr, Some(f));
        let stop = now.elapsed().unwrap().as_micros();
        return stop;


}

const SIZE: usize = 100;
const INT_MAX:i32 = 2>>20;
const N: u128 = 100;
fn main() {
    let fns = [lib::DefaultPartition::<i32>, lib::HoarePartition::<i32>, lib::MedianOfThreePartition::<i32>];
    let mut avg: [u128;3] = [0,0,0];
    for _ in  0..N{
        let v: Vec<i32> = (0..SIZE).map(|_| rand::thread_rng().gen_range(0..=100)).collect();
        
        for i in 0..fns.len(){
            println!("sorting on alg: {}", i);
            let mut v_temp = v.clone();
            avg[i] += compute_and_run(&mut v_temp, fns[i]);
        }
    }
    for i in 0..avg.len(){
        avg[i] /= N;
    }
    println!("results:\n \
default: {}  us\n\
Hoare: {} us\n \
MedianOfThree: {} us\n", avg[0], avg[1], avg[2]);
}
