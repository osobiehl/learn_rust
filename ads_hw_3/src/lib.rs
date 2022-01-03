pub fn selection_sort<T: PartialOrd + Clone>(arr: &mut [T])->(){

    println!("size of arr: {}", arr.len() );

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

