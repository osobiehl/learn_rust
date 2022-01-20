use std::borrow::Borrow;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: std::fmt::Debug>{
    val: T,
    next: Link<T>
}

impl<T: std::fmt::Debug> Node<T>{
    pub fn new(val: T)->Node<T>{
        return Node{val, next: None}
    }
    pub fn set_prev(&mut self, mut n: Link<T>){

        
        if let Some(b) = n.take(){
            self.next = Some(b);
        }
        else{
            dbg!(n);
            panic!("link given is incorrect!");
        }
        
    }
}
struct Stack<T: std::fmt::Debug>{
    current_size: usize,
    size: i32,
    top: Link<T>
}

impl<T: std::fmt::Debug> Stack<T>{
    pub fn new()->Stack<T>{
        return Stack { top: None, size: -1, current_size: 0 }
    }
    pub fn with_capacity(size: i32)->Stack<T>{
        return Stack {top: None, size, current_size: 0};

    }
    fn set_head(&mut self, v:T)->Result<(),()>{
        self.current_size = 1;
        let mut n = Node::new(v);
        self.top = Some(Box::new(n));
        return Ok(());

        
    }
    fn _push(&mut self, val: T)->Result<(),()>{
        let mut n = Node::new(val);
        dbg!(&self.top);
        n.set_prev(self.top.take());
        self.top = Some(Box::new(n));
        self.current_size+=1;
        return Ok(());

    }
    pub fn push(&mut self, val: T)->Result<(), ()>{

        if self.size == self.current_size as i32 {
            return Err(());
        }
        dbg!(self.current_size);
        
        return match self.current_size {
            0 => self.set_head(val),
            _ => self._push(val),
            
        }
        
    }
    pub fn pop(&mut self)->Option<T>{

        match mem::replace(&mut self.top, None){
            None => None,
            Some(b) => {
                self.current_size -= 1;
                self.top = b.next;
                Some(b.val)
            }
        }


    }
    pub fn is_empty(&self)->bool{
        return self.current_size == 0;
    }
    
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_stack(){
        let mut s: Stack<i32> = Stack::new();
        for i in 0..=10{
            s.push(i);
        }
        for _ in 0..5{
            println!("{}", s.pop().unwrap());

        }

    }
    


}