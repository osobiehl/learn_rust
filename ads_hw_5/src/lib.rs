

pub fn fibonacci_naive( n: usize)->usize{
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_naive(n-2) + fibonacci_naive(n-1)
    }
}

pub fn fib_bottomup(n: usize)->usize{
    let mut i=0;
    let mut j=1;
    let mut k = 0;
    if n < 2 {return n;}
    for _ in 2..=n{
        k = i + j;
        i=j;
        j=k;
    }
    return k;
}

// pub fn fib_closed(n: u32) {
//     unsafe{
//      let ans =  1.0 / (5 as f64).sqrt() * (   f64::pow((1.0 + (5 as f64).sqrt()) / 2.0, n) +  f64::pow(((1.0 - (5 as f64).sqrt())/2.0 ), n  ) ) ;
//     }
// }

pub fn fib_matrix(n: usize)->usize{
    let f = fibMatrix::new();
    return f.pow(n).val();
}
use std::ops::{IndexMut, Index, Mul};
struct fibMatrix{
    rows: [fibRow; 2]
}

impl fibMatrix{
    fn new() ->fibMatrix{
        return fibMatrix {rows: [fibRow::new([1,1]) , fibRow::new([1,0]) ] }
    }
    fn identity() ->fibMatrix{
        return fibMatrix {rows: [fibRow::new([1,0]) , fibRow::new([0,1]) ] }
    }

    fn val(&self) ->usize{
        return self[0][1];
    }
    fn pow(&self, n: usize)->fibMatrix{
        if n == 1{
            return fibMatrix::new()
        }
        else if n == 0{
            return fibMatrix::identity()
        }
        let x = self.pow(n/2);
        let r = &x * &x;
        match n % 2 {
            0 => &x*&x,
            1 => &(&x*&x)*self,
            _ => panic!("case should not happen!")
        }
    }
    fn mult(&self, other: &fibMatrix)-> fibMatrix{
        let a1 = self[0][0] * other[0][0] + self[0][1] * other[1][0];
        let a2 = self[0][0] * other[0][1] + self[0][1] * other[1][1];
        let b1 = self[1][0] * other[0][0] + self[1][1] * other[1][0];
        let b2 = self[1][0] * other[0][1] + self[1][1] * other[1][1];
        return fibMatrix{ 
            rows: [fibRow::new([a1,a2]) , fibRow::new([b1,b2])]
        }
    }
}

impl Mul<&fibMatrix> for &fibMatrix{
    type Output = fibMatrix;
    fn mul(self, rhs: &fibMatrix)->fibMatrix{
        return self.mult(&rhs);
    }
}

impl Index<usize> for fibMatrix {
    type Output = fibRow;
    fn index<'a>(&'a self, index: usize) -> &'a fibRow {
        return &self.rows[index];
    }
}


impl IndexMut<usize> for fibMatrix {

    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut fibRow {
        return & mut self.rows[index];
    }
}

struct fibRow{
    values: [usize; 2]
}


impl fibRow{
    fn new(a: [usize; 2])->fibRow{
        return fibRow { values: a }
    }

    
}
impl Index<usize> for fibRow{
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.values[index];
        
    }
}
impl IndexMut<usize> for fibRow{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return & mut self.values[index];
    }
}


