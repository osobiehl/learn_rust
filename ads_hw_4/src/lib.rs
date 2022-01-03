pub fn merge_sort<T: PartialOrd + Copy + std::fmt::Debug>(arr: &mut [T],  k: usize){
    merge_sort_(arr, k);
}

fn merge_sort_<T: PartialOrd + Copy +std::fmt::Debug>(arr: &mut [T], k : usize){
   
    if  arr.len() <= k {
        selection_sort(arr);
    }
    else{
        let mid = arr.len() / 2;
        let (a1, a2) = arr.split_at_mut(mid);
        merge_sort_(a1, k);
        merge_sort_(a2, k);
        let a1 = a1.to_owned();
        let a2 = a2.to_owned();
        merge(arr, &a1, &a2);

    }
        
}

fn merge<T: PartialOrd + Copy + std::fmt::Debug>(out: &mut [T], a1: & [T], a2: & [T]) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut o: usize = 0;
    while i < a1.len() && j < a2.len() {

        

        if a2[j] < a1[i]{
            out[o] = a2[j];
            j += 1;
            o += 1;
        }
        else {
            out[o] = a1[i];
            i += 1;
            o += 1;
        
        }


    }
    // dbg!(&a1);
    // dbg!(&a2);
    // println!("i: {}, j: {}, o: {}", &i,&j,&o);
    
    
    while i < a1.len(){
        out[o] = a1[i];
        i += 1;
        o += 1;
    }
    while j < a2.len() {
        out[o] = a2[j];
        j += 1;
        o += 1;
    }
    // dbg!(& out);


}

pub fn selection_sort<T: PartialOrd + Clone>(arr: &mut [T])->(){

    // println!("size of arr: {}", arr.len() );

    let l = arr.len();
    for i in 0..l{
        let smallest = extract_min_pos(&mut arr[i..l] );
        arr.swap(i, i + smallest);
        
    }



}

fn extract_min_pos<T: PartialOrd + Clone>(arr: &mut [T])->usize{
    let mut idx= 0;
    let mut min = arr[0].clone();
    for (i, v) in arr.into_iter().enumerate(){
        if *v <  min {

            idx = i;
            min = v.clone()
        }
    }
    return idx;
}