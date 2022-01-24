use std::str::ParseBoolError;


struct Heap<'a, T> where
T: PartialOrd + Copy + std::fmt::Debug{
    arr: &'a mut [T],
    size: usize
}
pub fn heap_sort<'a, T>(arr: &'a mut [T]) where
T: PartialOrd + Copy + std::fmt::Debug{
    let mut  H = Heap::from(arr);
    H.sort();

}
pub fn heap_sort_bottomup<'a, T>(arr: &'a mut [T]) where
T: PartialOrd + Copy + std::fmt::Debug{
    let mut  H = Heap::from(arr);
    H.bottom_up_sort();
}

struct Min_Queue<T: PartialOrd+ Copy>{
    arr: Vec<T>

}

impl<T>  Min_Queue<T> where
T: PartialOrd + Copy{
    
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

    fn fix_up(&mut self, val_idx: usize, mut float_idx: usize)->usize{
        let  a =  & mut self.arr;
        while a[val_idx] > a[float_idx] {
            float_idx = self.parent(float_idx);
        }
        let mut temp = a[float_idx];
        a[float_idx] = a[val_idx];
        fn swap<T: Copy>(a: &mut T, b: &mut T){
            let temp = *a;
            *a = *b;
            *b = temp;
        }
        while float_idx > val_idx{
            float_idx = Heap::<T>::parent(float_idx);
            swap(&mut a[float_idx], &mut temp)
        }



        
        
        return 0
    }

}

impl<T: PartialOrd + Copy> PriorityQueue<T> for Min_Queue<T>{

    fn from(arr: &mut [T])->Self{
        let mut v = vec![];
        v.copy_from_slice(arr);
        let H = Min_Queue{
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


}
trait PriorityQueue<T>{
    fn insert(val: T);
    fn extract()->T;
    fn heapify(&mut self);
    fn from(arr: & mut [T])->Self;
    fn sink(&mut self, indx: usize);
    fn max(&self, l: usize, r: usize)->usize;
    fn parent(idx: usize)->usize;


}

impl<'a,T>  Heap<'a,T> where
T: PartialOrd + Copy + std::fmt::Debug{
    fn from(arr: &'a mut [T])->Heap<'a, T>{
        let l = arr.len();
        
        let mut H = Heap {arr, size: l};
        H.heapify();
        return Heap { arr, size: l };
    }
    fn heapify(&mut self){
        let mid = self.arr.len() / 2;
        // dbg!(mid);
        for i in (0..mid).rev(){
            // dbg!(i);
            self.sink(i);
        }

    }
    fn sink(&mut self, indx: usize){
        let x = self.children(indx);
        // dbg!(x);
        
        if let (Some(x), Some(y)) = x{
            let a =&mut self.arr;
            let (g, g_i) = if a[x] > a[y] {(a[x],x)} else {(a[y], y)};
            if a[g_i] > a[indx]{
                a.swap(g_i, indx);
                self.sink(g_i);
            }
        }
        else if let(Some(x), None ) = x{
            let a =&mut self.arr;   
            if a[x] > a[indx]{
                a.swap(x, indx);

            }

        }
        
    }
    pub fn sort(&mut self){
        for i in (1..self.arr.len()).rev(){
            self.arr.swap(0, i);
            self.size -= 1;
            self.sink(0);
            // dbg!(&self.arr);
        }
        self.size = self.arr.len();


}    
    fn max(&self, l: usize, r: usize)->usize{
        return if self.arr[l] > self.arr[r] {l} else {r}
    }
    fn float_down(&mut self, idx: usize)->usize{
        let mut i = idx;
        loop{
            let mut c =  self.children(i);
            i = match c{
                (Some(x), Some(y)) => self.max(x,y),
                (Some(x), None) => x,
                _ => break (),

            }

        };
        return i;
    }



    fn fix_up(&mut self, val_idx: usize, mut float_idx: usize)->usize{
        let  a =  & mut self.arr;
        while a[val_idx] > a[float_idx] {
            float_idx = Heap::<T>::parent(float_idx);
        }
        let mut temp = a[float_idx];
        a[float_idx] = a[val_idx];
        fn swap<T: Copy>(a: &mut T, b: &mut T){
            let temp = *a;
            *a = *b;
            *b = temp;
        }
        while float_idx > val_idx{
            float_idx = Heap::<T>::parent(float_idx);
            swap(&mut a[float_idx], &mut temp)
        }



        
        
        return 0
    }

    pub fn parent(idx: usize)->usize{
        return (idx-1)/2;
    }

    pub fn bottom_up_sort(&mut self){
        for i in (1..self.arr.len()).rev(){
            self.arr.swap(0, i);
            self.size -= 1;
            let idx = self.float_down(0);
            self.fix_up(0, idx);
            
            // dbg!(&self.arr);
        }
        self.size = self.arr.len();
    }
    fn children(&self, indx: usize)->(Option<usize>, Option<usize>){
        let c = indx * 2 + 1;
        if c <self.size -1{
            return (Some(c), Some(c+1));
        }
        if c >= self.size{
            return (None, None);
        }
        else {
            return (Some(c), None)
        }

    }
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
    fn test(){
        let mut v = vec![1,9,0,2,8,3,4,5,7,6,7,7];
        bubble_sort(&mut v);
        assert!(is_sorted(&mut v));
    }
    #[test]
    fn is_heap(){
        let mut v = vec![1,9,0,2,8,3,4,5,7,6,7,7];
        dbg!(&v);
        let H = Heap::from(&mut v);

        dbg!(&H.arr);
    }
    #[test]
    fn heap_test(){
        let mut v = vec![1,9,0,2,8,3,4,5,7,6,7,7];
        heap_sort(&mut v);
        dbg!(&v);
        assert!(is_sorted(&mut v));
    }
    #[test]
    fn heap_test_bottomup(){
        let mut v = vec![1,9,0,2,8,3,4,5,7,6,7,7];
        heap_sort_bottomup(&mut v);
        dbg!(&v);
        assert!(is_sorted(&mut v));
    }
    fn test_djikstra(){
        Djikstra();
    }

}

pub fn Djikstra<T: Sized + std::fmt::Debug>(D: &mut [Vec<T>]){
    for (i, V) in D.iter().enumerate(){

    }

}
