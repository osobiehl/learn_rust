#[derive(Clone, Copy)]
struct Dynamic{
    prev: Option<usize>,
    len: usize,
}

fn walk_back(V: &Vec<Dynamic>,mut d: (usize, &Dynamic))->Vec<usize>{
    let mut ans = vec![];
    let (idx, mut v) = d;
    ans.push(idx);
    while let Some(i) = v.prev{
        ans.push(i);
        v =& V[i];

    }
    return ans;
}
pub fn longest_ordered_subarray<T: PartialOrd + Copy>(arr: &mut [T])->Vec<T>{
    //use prev instead of list for each
    // use a tuple: length + prev choice

    let mut DP: Vec<Dynamic> = vec![Dynamic{prev: None, len: 1}; arr.len()];
    let l = arr.len();
    for i in (1..l){
        for j in (0..i).rev(){
            //case: the value we find a smaller value
            if arr[j] < arr[i]{
                if DP[j].len +1 > DP[i].len{
                    DP[i].prev = Some(j);
                    DP[i].len = DP[j].len + 1;
                } 
            }
        }
    }
    //find solution
    let max = DP.iter().enumerate().reduce(|accum, item|{
        if item.1.len > accum.1.len {item} else {accum}
    }).unwrap();
    let indexes = walk_back(&DP, max);
    let ans = indexes.iter().map(|x| arr[*x]).collect();
    return ans;


}

type pyramid = Vec<Vec<usize>>;
#[derive(Debug, Clone, Copy)]
pub enum Dir{
    L,
    R
}
pub fn pyramid_solution(P: &pyramid)->Vec<(usize, Dir)>{
    let mut P_flipped = P.iter().rev();

    let mut opt: Vec<Vec<(Dir, usize)>> = Vec::with_capacity(P.len());
    let first_row = P_flipped.next().unwrap()
    .iter().map(|i| (Dir::L, *i)).collect();
    opt.push( first_row) ;
    let mut prev_row_num = 0;
    for row in P_flipped{

        let mut opt_row:Vec<(Dir, usize)> = Vec::with_capacity(row.len());
        let prev_opt_row = &opt[prev_row_num];
        
        for (idx, val) in row.iter().enumerate(){
            
            if (prev_opt_row[idx].1 > prev_opt_row[idx+1].1){
                opt_row.push((Dir::L, prev_opt_row[idx].1 + * val))
            }
            else{
                opt_row.push((Dir::R, prev_opt_row[idx+1].1 + * val))
            }

        }
        opt.push(opt_row);
        prev_row_num +=1;
    }
    let mut i = 0;
    let mut res: Vec<(usize, Dir)> = vec![];
    for (idx,v) in opt.iter().rev().enumerate(){
        let item = &v[i];
        res.push((P[idx][i], item.0));
        match item.0{
            Dir::L => (),
            Dir::R => i +=1
        };



    }
    dbg!(&res);
    return res;

    
}
#[cfg(test)]
mod tests{
    use std::vec;

    use super::*;
    #[test]
    fn test(){
        let mut v = vec![8,3,6,50,10,8,100,30,60,40,80];
        let vals = longest_ordered_subarray(&mut v);
        dbg!(&vals);
    }
    #[test]
    fn pyramid_test(){
        let mut vals = vec![
            vec![7],
            vec![3,8],
            vec![8,1,0],
            vec![2,7,4,4],
            vec![4,5,2,6,5]
        ];
        pyramid_solution(&mut vals);
    }
    
}