

use rand::prelude::*;
/// uses the first element in the array
pub fn DefaultPartition<T: Copy + PartialOrd >(arr: & mut [T])->usize{
    let pivot = arr[0];
    let mut p: usize = 1;
    let l = arr.len();
    for i in 1..l{
        if arr[i] <= pivot{
            arr.swap(i, p);
            p+= 1;
        }
    }
    arr.swap(0, p-1);
    return p-1;


}

fn median_index<T: PartialOrd>(arr: &[T])->usize{
    let a:& T;
    let b:& T;
    let c:& T;
    let m = arr.len()/2;
    a = &  arr[0];
    b = &  arr[m];
    c = & arr[arr.len()-1];

    if  (*a > *b) ^ (*a > *c) {
        return 0;
    }
    else{
        if (*a > *b) ^ (* b > *c ) {
            return arr.len() -1
        }
        else {return m}
    }

}

pub fn MedianOfThreePartition
<T: Copy + PartialOrd >(arr: & mut [T])->usize{
    
    let p = median_index(arr);
    arr.swap(0,p);

    return DefaultPartition(arr);
}

pub fn HoarePartition<T: Copy + PartialOrd>(arr: &mut [T])->usize{

    let pivot = arr[0];
    let mut hi = arr.len() -1;
    let mut lo = 1;

    

    while lo < arr.len() && arr[lo] < pivot {
        lo+=1;
    }
    while arr[hi] > pivot{
        hi-=1;
    }

    if lo >= hi {arr.swap(0,hi); return hi}
    arr.swap(lo, hi);
    
    loop{
        lo+=1;
        while arr[lo] < pivot{
            lo+=1;
        }
        hi-= 1;
        while arr[hi] > pivot{
            hi-=1;
        }

        if lo >= hi {arr.swap(0,hi); return hi}
        arr.swap(lo, hi)
    }
}
pub fn QuickSort< T: Copy + PartialOrd >(arr: &mut [T], partition: Option<fn(& mut [T]) -> usize>)  {
    if arr.len() <= 1{
        return;
    }
    let partition = partition.unwrap_or(DefaultPartition );
    let m = partition(arr);
    let (arr1, temp) = arr.split_at_mut(m);
    if arr1.len() > 1{
        QuickSort(arr1, Some(partition));
    }
    if temp.len() > 2{
        QuickSort(&mut temp[1..], Some(partition))
    }
}
pub fn quick_sort_3<T: Copy + PartialOrd + std::fmt::Debug + std::fmt::Display>(arr: &mut [T]){
    let l = arr.len() -1;
    quick_sort_3_(arr, 0, l);
    
}
fn quick_sort_3_<T: Copy + PartialOrd + std::fmt::Debug + std::fmt::Display>(arr: &mut [T],  lo: usize,  hi: usize){
    if hi <= lo{return;}
    // if lo + 2 == hi {
    //     partition_3(arr, lo, hi);
    // }
    else{


        let (lo_m, hi_m) = partition_3(arr, lo, hi);
        if  lo_m == hi_m { return;}
        quick_sort_3_(arr, lo, lo_m);
        quick_sort_3_(arr, lo_m+1, hi_m);
        quick_sort_3_(arr, hi_m+1, hi)
    
    }
}
pub fn random_quick_sort_3<T: Copy + PartialOrd + std::fmt::Debug + std::fmt::Display>(arr: &mut [T]){
    r_qsort_3(arr, 0, arr.len()-1);

}
fn  r_qsort_3<T: Copy + PartialOrd + std::fmt::Debug + std::fmt::Display>(arr: &mut [T],  lo: usize,  hi: usize){
    if hi <= lo{return;}
    // if lo + 2 == hi {
    //     partition_3(arr, lo, hi);
    // }
    else{


        let (lo_m, hi_m) = partition_3_rand(arr, lo, hi);
        if  lo_m == hi_m { return;}
        quick_sort_3_(arr, lo, lo_m);
        quick_sort_3_(arr, lo_m+1, hi_m);
        quick_sort_3_(arr, hi_m+1, hi)
    
    }
}
fn partition_3_rand<T: PartialOrd + Copy + std::fmt::Debug + std::fmt::Display>(arr: &mut [T], lo: usize, hi: usize)->(usize, usize){
    let l = hi - lo;
    if l > 2{
        random_swap(arr,lo+1,hi);
        random_swap(arr, lo, hi)
    } 
    return partition_3(arr,lo,hi);
}

fn random_swap<T: PartialOrd + Copy + std::fmt::Debug + std::fmt::Display>(arr: &mut [T], lo: usize, hi: usize){
    let r = rand::thread_rng().gen_range(lo..=hi);
    arr.swap(lo, r);
}

fn partition_3<T: PartialOrd + Copy + std::fmt::Debug + std::fmt::Display>(arr: &mut [T], lo: usize, hi: usize)->(usize, usize){
    if lo == hi {return (0,0)}
    let l = hi-lo;
    if arr[lo] > arr[lo+1] {
        arr.swap(lo,lo+1);
    }
    if l == 1{ return (lo, hi)}
    if l == 2{
        // println!("case l == 3!");
        if arr[lo+2] < arr[lo+1]{
            if arr[lo] > arr[lo+2]{
                arr.swap(lo, lo+2);

                
            }
            arr.swap(lo+1, lo+2)

            
        }
        // dbg!(&arr[lo..=hi]);
        return (0, 0)
    }
    if arr[lo] > arr[lo+1] {
        arr.swap(lo,lo+1);
    }

    let sm = arr[lo];
    let lg = arr[lo+1];

    let (mut p, mut q) = (lo+2,lo+2);

    for i in lo+2..=hi{
       if arr[i] < lg {
            if arr[i] < sm {
                arr.swap(p, i);
                if p != q{
                    arr.swap(q,i);
                }
                p+=1; q +=1;
                
            }
            else{
                arr.swap(q, i);
                q += 1;
            }
       }
    }
    assert!(p<=q);
    // if (p == q && p == lo+2){println!("lready sorted");return (0,0)}
    arr.swap(lo+1, p-1); //swap
    arr.swap(p-1, q-1);
    arr.swap(lo, p-2);

    
    
    return (p-2,q-1)



}

#[cfg(test)]
mod tests {
    use std::vec;

    use rand::random;

    use super::*;
    #[test]
    fn correct_partition(){
        let mut v = vec![4,6,2,7,3];
        let ans = DefaultPartition(&mut v);
        assert_eq!(v, vec![3,2,4,7,6]);
        assert_eq!(ans, 2)

    }
    #[test]
    fn correct_MedianOfThreeIndex(){
        let mut v = vec![4,6,2,7,3];
        let ans = median_index(&mut v);
        assert_eq!(ans, 4);

    }

    #[test]
    fn correct_HoareSort(){

        let v = vec![1,2,3,4,5,6,7,8,9];
        let mut v1 = v.clone();
        QuickSort(&mut v1, Some(HoarePartition) );
        assert_eq!(v1, v);
        let mut v2 = vec![7,9,5,8,6,4,1,3,2];
        QuickSort(&mut v2, Some(HoarePartition));
        assert_eq!(v2, v);


    }

    #[test]
    fn correct_MedianOfThreeSort(){

        let v = vec![1,2,3,4,5,6,7,8,9];
        let mut v1 = v.clone();
        QuickSort(&mut v1, Some(MedianOfThreePartition) );
        assert_eq!(v1, v);
        let mut v2 = vec![7,9,5,8,6,4,1,3,2];
        QuickSort(&mut v2, Some(MedianOfThreePartition));
        assert_eq!(v2, v);


    }


    #[test]
    fn correct_quicksort(){
        let v = vec![1,2,3,4,5,6,7,8,9];
        let mut v1 = v.clone();
        QuickSort(&mut v1, None);
        assert_eq!(v1, v);
        let mut v2 = vec![7,9,5,8,6,4,1,3,2];
        QuickSort(&mut v2, None);
        assert_eq!(v2, v);
    }

    #[test]
    fn correct_3_partition(){
        let mut v = vec![4,6,2,7,3];
        let l = v.len()-1;
        let ans = partition_3(&mut v, 0, l);
        assert_eq!(v, vec![2,3,4,6,7]);
        assert_eq!(ans, (2,3));
        let mut v = vec![4,8,3,9,6,1,5,6,2];
        let l = v.len()-1;
        
        let ans = partition_3(&mut v, 0, l);
        dbg!(&v);
        // assert_eq!(v, vec![2,3,4,6,7]);
        assert_eq!(ans, (3,7))

    }

    #[test]
    fn correct_3_sort(){                                                                              fn correct_quicksort_3(){
        let v = vec![1,2,3,4,5,6,7,8,9];
        let mut v1 = v.clone();
        quick_sort_3(&mut v1);
        assert_eq!(v1, v);
        let mut v2 = vec![7,9,5,8,6,4,1,3,2];
        quick_sort_3(&mut v2);
        assert_eq!(v2, v);
        let mut v: Vec<i32> = (0..100).collect();

        v.shuffle(&mut thread_rng());
        let mut vc = v.clone();
        quick_sort_3(&mut vc);
        dbg!(&v);
        dbg!(&vc);
        assert!(vc.windows(2).all(|x| {if x[0] > x[1]{
            println!("error at: {} {}", x[0], x[1]);
            return false;
        }return true;}))
        
    }
}

#[test]
fn correct_3_rand_sort(){                                                                              fn correct_quicksort_3(){
    let v = vec![1,2,3,4,5,6,7,8,9];
    let mut v1 = v.clone();
    random_quick_sort_3(&mut v1);
    assert_eq!(v1, v);
    let mut v2 = vec![7,9,5,8,6,4,1,3,2];
    random_quick_sort_3(&mut v2);
    assert_eq!(v2, v);
    let mut v: Vec<i32> = (0..100).collect();

    v.shuffle(&mut thread_rng());
    let mut vc = v.clone();
    random_quick_sort_3(&mut vc);
    dbg!(&v);
    dbg!(&vc);
    assert!(vc.windows(2).all(|x| {if x[0] > x[1]{
        println!("error at: {} {}", x[0], x[1]);
        return false;
    }return true;}))
    
}
}
        
}