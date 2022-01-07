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

    // match arr.len(){
    //     1 => return 0,
    //     2 => {if pivot > arr[1]{ arr.swap(0,1); return 1} else {return 0}}
    //     _ => ()

    // }

    dbg!(lo, hi);
    

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

#[cfg(test)]
mod tests {
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

}