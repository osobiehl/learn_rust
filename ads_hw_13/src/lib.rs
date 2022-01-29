use std::{str::ParseBoolError, f64::MIN, cmp::Ordering};


struct Heap<'a, T> where
T: PartialOrd + Copy + std::fmt::Debug{
    arr: &'a mut [T],
    size: usize
}
// pub fn heap_sort<'a, T>(arr: &'a mut [T]) where
// T: PartialOrd + Copy + std::fmt::Debug{
//     let mut  H = Heap::from(arr);
//     H.sort();

// }
// pub fn heap_sort_bottomup<'a, T>(arr: &'a mut [T]) where
// T: PartialOrd + Copy + std::fmt::Debug{
//     let mut  H = Heap::from(arr);
//     H.bottom_up_sort();
// }

struct Min_Queue<T: PartialOrd+ Copy>{
    arr: Vec<T>

}
fn parent(idx: usize)->usize{
    return (idx-1)/2;
}

impl<T>  Min_Queue<T> where
T: PartialOrd + Copy{
    fn new()->Self{
        Min_Queue{arr: vec![]}
    }
    
    fn children(&self, indx: usize)->(Option<usize>, Option<usize>){
        let c = indx * 2 + 1;
        let size = self.arr.len();
        if c <size -1{
            return (Some(c), Some(c+1));
        }
        if c >= size{
            return (None, None);
        }
        else {
            return (Some(c), None)
        }

    }
    fn insert_fix_up(&mut self){
        let mut v = self.arr.len() -1;
        let mut p = parent(v);
        while self.arr[v] < self.arr[p]{

            self.arr.swap(v, p);
            if p == 0{break;}
            v = parent(v);
            p = parent(v);
        }
    }

    fn fix_up(&mut self, val_idx: usize, mut float_idx: usize)->usize{
        let  a =  & mut self.arr;
        while a[val_idx] > a[float_idx] {
            float_idx = parent(float_idx);
        }
        let mut temp = a[float_idx];
        a[float_idx] = a[val_idx];
        fn swap<T: Copy>(a: &mut T, b: &mut T){
            let temp = *a;
            *a = *b;
            *b = temp;
        }
        while float_idx > val_idx{
            float_idx = parent(float_idx);
            swap(&mut a[float_idx], &mut temp)
        }



        
        
        return 0
    }

}

impl<T: PartialOrd + Copy> PriorityQueue<T> for Min_Queue<T>{

    fn from_arr(arr: &mut [T])->Self{
        let mut v = Vec::new();
        v.extend_from_slice(arr);
        let mut H = Min_Queue{
            arr: v
        };
        H.heapify();
        return H;

    }

    
    fn heapify(&mut self) {
        let mid = self.arr.len() / 2;
        // dbg!(mid);
        for i in (0..mid).rev(){
            // dbg!(i);
            self.sink(i);
        }
    }
    fn sink(&mut self, indx: usize) {
        let x = self.children(indx);
        // dbg!(x);
        
        if let (Some(x), Some(y)) = x{
            let a =&mut self.arr;
            let (g, g_i) = if a[x] < a[y] {(a[x],x)} else {(a[y], y)};
            if a[g_i] < a[indx]{
                a.swap(g_i, indx);
                self.sink(g_i);
            }
        }
        else if let(Some(x), None ) = x{
            let a =&mut self.arr;   
            if a[x] < a[indx]{
                a.swap(x, indx);

            }

        }
    }
    fn insert(&mut self, val: T){
        self.arr.push(val);
        self.insert_fix_up();
    }
    fn extract(&mut self) ->T {

        let val = self.arr[0];
        let i = self.arr.len()-1;
        self.arr.swap(0, i);
        self.arr.pop();
        self.sink(0);
        
        return val;
        
    }


}
trait PriorityQueue<T>{
    fn insert(&mut self, val: T);
    fn extract(&mut self)->T;
    fn heapify(&mut self);
    fn from_arr(arr: & mut [T])->Self;
    fn sink(&mut self, indx: usize);
    // fn max(&self, l: usize, r: usize)->usize;
    // fn parent(idx: usize)->usize;


}




pub fn bubble_sort<T>(arr: &mut [T])
where T: PartialOrd + std::fmt::Debug + Copy{
    for i in 0..arr.len(){
        let mut swapped = false;
        for j in 1..arr.len(){
            if arr[j] < arr[j-1]{
                swapped = true;
                arr.swap(j, j-1);
            }
        }
        if swapped == false{break;}
    }

}
pub fn is_sorted<T>(arr: &mut [T])->bool
where T: PartialOrd + std::fmt::Debug + Copy{
    return arr.windows(2).all(|x|  
        x[0] <= x[1]
    );
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn is_heap(){
        let mut v = vec![1,9,0,2,8,3,4,5,7,6,7,7];
        dbg!(&v);
        let H = Min_Queue::from_arr(&mut v );

        dbg!(&H.arr);
    }
    fn test_heap<T>(H: & Min_Queue<T>, idx: usize )
    where T: Copy + std::fmt::Display+ std::fmt::Debug + PartialOrd{
        let a = &H.arr;
        match H.children(idx){
            (Some(x), Some(y)) => {
                if a[x] < a[idx] || a[y] < a[idx]{
                    dbg!(a);
                    panic!("error at: {},{}",x,y);
                }
                println!("testing on: {}", &idx);
                test_heap(&H,x);
                test_heap(&H,y);
            }
            (Some(x), None) => {
                if a[x] < a[idx]{
                    dbg!(a);
                    panic!("error at: {}", x);
                }
                println!("testing on: {}", &idx);
                test_heap(&H,x);
            }
            _ => ()
        }

    }
    #[test]
    fn heap_test(){
        let mut v = vec![1,9,0,2,8,3,4,5,7,6,7,7];
        let H = Min_Queue::from_arr(&mut v);
        test_heap(&H, 0);

    }
    #[test]
    fn test_insert(){
        let mut v = vec![1,9,0,2,8,3,4,5,7,6,7,7];
        let mut H = Min_Queue::from_arr(&mut v);
        H.insert(-45);
        H.insert(100);
        H.insert(7);
        test_heap(&H, 0);
    }

    fn test_djikstra(){
        // Djikstra();
    }

}
#[derive(Clone, Copy, Debug)]
struct Djikstra_Edge{
    E: (usize, usize),
    W: usize
}
impl PartialEq for Djikstra_Edge{
    fn eq(&self, other: &Self) -> bool {
        return self.W == other.W;
    }
}
impl PartialOrd for Djikstra_Edge{
    fn partial_cmp(&self, other: &Self)->Ordering{
        return self.W.cmp(&other.W);
    }
    fn lt(&self, other: &Self) -> bool {
        return self.W < other.W;

    }
    fn le(&self, other: &Self) -> bool {
        return self.W <= other.W;
    }
    fn ge(&self, other: &Self) -> bool {
        return self.W >= other.W;
    }
    fn gt(&self, other: &Self) -> bool {
        return self.W > other.W;
    }
}

pub fn Djikstra<T: Sized + std::fmt::Debug>(D: &mut [Vec<T>]){
    let min_queue: Min_Queue<Djikstra_Edge> = Min_Queue::new()
    for (i, V) in D.iter().enumerate(){
        

    }

}
