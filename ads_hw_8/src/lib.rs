use core::num;


fn extract_max<T: Copy + PartialOrd>(arr: & [T])->T{
    let mut max =  arr[0];
    for i in arr{
        if *i > max{
            max = *i;
        }
    }
    return max;
}

pub fn counting_sort(arr: &mut [usize]){

    counting_sort_(arr, extract_max::<usize>, |i| *i);
}
pub fn counting_sort_<T: Copy, max_extract, indx_val >(arr: &mut [T], max: max_extract, indx: indx_val) 
where max_extract: Fn(&[T])->usize,
    indx_val: Fn(&T)->usize,
{
    let m = max(arr) + 1;
    // dbg!(m);
    let mut v: Vec<usize> = Vec::with_capacity(m);
    for _ in 0..m{
        v.push(0);
    }
    for i in arr.iter_mut(){
        v[indx(i)]+=1;
    }
    for i in 1..v.len(){
        v[i] += v[i-1];
    }
    let l = arr.len();
    // dbg!(l);
    let mut ans: Vec<T> = Vec::with_capacity(l);
    unsafe{
        ans.set_len(l);
    }
    for i in arr.into_iter().rev(){
        ans[v[indx(i)]-1] = *i;
        v[indx(i)] -= 1;
    }
    for i in 0..arr.len(){
        arr[i] = ans[i];
    }
    
}

pub fn counting_sort_splits<T: Copy, max_extract, indx_val >(arr: &mut [T], max: max_extract, indx: indx_val)->Vec<usize>
where max_extract: Fn(&[T])->usize,
    indx_val:  Fn(&T)->usize,
{
    let m = max(arr) + 1;
    // dbg!(m);
    let mut v: Vec<usize> = Vec::with_capacity(m);
    for _ in 0..m{
        v.push(0);
    }
    for i in arr.iter_mut(){
        v[indx(i)]+=1;
    }
    let res = v.to_owned();
    for i in 1..v.len(){
        v[i] += v[i-1];
    }
    let l = arr.len();
    // dbg!(l);
    let mut ans: Vec<T> = Vec::with_capacity(l);
    unsafe{
        ans.set_len(l);
    }
    for i in arr.into_iter().rev(){
        ans[v[indx(i)]-1] = *i;
        v[indx(i)] -= 1;
    }
    for i in 0..arr.len(){
        arr[i] = ans[i];
    }
    return res;
    
}

fn is_sorted<T: PartialOrd + std::fmt::Display>(arr: &mut [T])->bool{
    return arr.windows(2).all(|x| {if x[0] > x[1]{
        println!("error at: {} {}", x[0], x[1]);
        return false;
    }return true;})
}



fn bucket_sort(arr: &mut [f64], num_buckets: usize)->Vec<f64>{
    if arr.len() == 0{
        return arr.to_owned();
    }
    else if arr.windows(2).all(|x| x[0] == x[1]){
        return arr.to_owned();
    }
    let max = extract_max(arr);
    if max > 1.0{
        for i in &mut * arr{
            *i /= max;
        }
    }
    let mut buckets: Vec<Vec<f64>> = vec![[].to_vec(); num_buckets];
    for i in arr{
        buckets[  (*i * (num_buckets as f64)) as usize ].push(*i);
    }
    let mut ans: Vec<Vec<f64>> = vec![[].to_vec();num_buckets];
    for mut b in buckets{
        ans.push(bucket_sort(&mut b, num_buckets));
    }
    let mut result: Vec<f64> = vec![];
    for mut a in ans{
        result.append(&mut a);
    }
    if max > 1.0{
        for i in result.iter_mut(){
            *i *= max;
        }
    }
    return result;
}



pub fn radix_sort_<'a, T: Copy + PartialEq>(mut arr : &'a mut [T], idx: usize,
     map_fn: &Box< &dyn Fn(usize) ->  Box<  dyn  Fn(&T) -> usize> >) 
    {

        if arr.len() == 0 {return;}
        let m_ = &map_fn;
        let f = m_(idx);
        let f_ = &f;

        let max_extract = |arr: &[T]| {
            let mut max = f_(&arr[0]);
            for x in arr{
                if f(x) > max{
                    max = f(x);
                }
            }
            return max
        };

        if arr.windows(2).all(|x| {x[0]== x[1]}){
            return;
        }
        
        f(&arr[0]);
        let mut splits = counting_sort_splits( arr, max_extract, f_);
        let splits: Vec<usize> = splits.into_iter().filter(|x| *x> 0).collect();
        let mut refs: Vec<&mut [T]> = Vec::with_capacity(  splits.len() );

        
        for i in splits{
            let (v,  next)  = arr.split_at_mut(i);
            refs.push(v);
            arr = next;


        }
        refs.push(arr);

        for r in refs{
            
            radix_sort_(r, idx+1, map_fn);
        }

    }



// fn string_map<'r, 's>(i: usize)->Box<dyn Fn(&'r &'s str)->usize >{
//     return Box::new(|x: &'r &'s str| {
//         return x.as_bytes()[i] as usize;
//     })
// }

pub fn radix_sort_numbers(arr: &mut[&str]){
    radix_sort_(arr, 0, &Box::new(&|i: usize| {
        return Box::new( move
            |s: &&str| {
                if i == 0 {
                    //assume no preceding 0s
                    return s.len();
                }
                if i > s.len() {
                    return 0;
                }
                return s.as_bytes()[i-1] as usize;
            }
        )
        
        
    }) );

}

pub fn radix_sort_string(arr: & mut[& str]){
    // let map = ;
    radix_sort_(arr, 0, &Box::new(&|i: usize| {
        return Box::new( move
            |s: &&str| {
                if i >= s.len() {
                    return 0;
                }
                return s.as_bytes()[i] as usize;
            }
        )
        
        
    }) );
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        let mut v = vec![1,8,5,4,7,4,3,6,8,5,4,12,34,0,1,5];
        counting_sort(&mut v);
        assert!(is_sorted(&mut v));

    }
    #[test]
    fn test_bucket(){
        let mut v = vec![0.9, 0.1, 0.6, 0.7, 0.6, 0.3, 0.1];
        let mut ans = bucket_sort(&mut v, 10);
        dbg!(&ans);
        assert!(is_sorted(&mut ans));
    }
    #[test]
    fn test_radix(){
        let mut v = vec!["aaaa", "bbbbb", "aaabbbb", "ccccc", "aaaaaabc"];
        radix_sort_string(&mut v);
        dbg!(&v);
        assert!(is_sorted(&mut v));
    }
    #[test]
    fn test_radix_words(){
        let mut v = vec![ "9111", "9211",  "11111","1999","9299",];
        radix_sort_numbers(&mut v);
        dbg!(&v);
        assert_eq!(&v, &vec!["1999", "9111", "9211", "9299", "11111"]);
    }
}