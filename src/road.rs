#![allow(non_snake_case, unused_macros)]

use proconio::{input, marker::Usize1};
use std::collections::HashMap;
pub type Output = Vec<usize>;

#[derive(Clone, Debug)]
pub struct Input {
    pub D: usize,
    pub K: usize,
    pub es: Vec<(usize, usize, i32)>,
}

fn parse_input() -> Input {
    input! {
        _N: usize, M: usize, D: usize, K: usize,
        es: [(Usize1, Usize1, i32); M],
    }
    Input { D, K, es }
}

fn write(output: &Output) -> String {
    let x: Vec<String> = output.iter().map( |&id| id.to_string()).collect();
    x.join(" ")
}

fn next(d:usize,D:usize,tally:&Vec<HashMap<usize,usize>>,a:usize,b:usize,tots:&Vec<usize>,L:usize) -> usize {
   if L==0 {
       return d
   }
   if tots[d]<1 { 
       if d<(D-1) {
         return next(d+1,D,&tally,a,b,&tots,L)
       } else { 
         return next(0,D,&tally,a,b,&tots,L)
       }
   }
   match (tally[d].get(&a),tally[d].get(&b)) {
       (None,None) => d,
       (_,_) => if d<(D-1) {
                    next(d+1,D,&tally,a,b,&tots,L-1)
                } else {
                    next(0,D,&tally,a,b,&tots,L-1)
                }
   }
}


fn sorted(input: &Input) -> Output {
    let M = input.es.len();
    let D = input.D;
    let K = input.K;
    let mut tally:Vec<HashMap<usize,usize>> = (0..D).map(|_| HashMap::new()).collect();
    let mut tots = vec![K; D];
    let mut ordered : Vec<(usize,&(usize,usize, i32))> = input.es.iter().enumerate().collect();
    ordered.sort_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap());
    let mut out= vec![0; M];
    for n in 0..M {
        let d = next(n%D,D,&tally,(ordered[n].1).0,(ordered[n].1).1,&tots,D);
        match tally[d].get(&(ordered[n].1).0) {
            Some(&j) => tally[d].insert((ordered[n].1).0,j+1),
            _ => tally[d].insert((ordered[n].1).0,1),
        };
        match tally[d].get(&(ordered[n].1).1) {
            Some(&j) => tally[d].insert((ordered[n].1).1,j+1),
            _ => tally[d].insert((ordered[n].1).1,1),
        };
        out[ordered[n].0]=1+d;
        tots[d] -= 1;
    }
    out
}

pub fn main() {
    let input = parse_input();
    let guess0 = sorted(&input);
    println!("{}", write(&guess0));
}
#![allow(non_snake_case, unused_macros)]

use proconio::{input, marker::Usize1};
use std::collections::HashMap;
pub type Output = Vec<usize>;

#[derive(Clone, Debug)]
pub struct Input {
    pub D: usize,
    pub K: usize,
    pub es: Vec<(usize, usize, i32)>,
}

fn parse_input() -> Input {
    input! {
        _N: usize, M: usize, D: usize, K: usize,
        es: [(Usize1, Usize1, i32); M],
    }
    Input { D, K, es }
}

fn write(output: &Output) -> String {
    let x: Vec<String> = output.iter().map( |&id| id.to_string()).collect();
    x.join(" ")
}

fn next(d:usize,D:usize,tally:&Vec<HashMap<usize,usize>>,a:usize,b:usize,tots:&Vec<usize>,L:usize) -> usize {
   if L==0 {
       return d
   }
   if tots[d]<1 { 
       if d<(D-1) {
         return next(d+1,D,&tally,a,b,&tots,L)
       } else { 
         return next(0,D,&tally,a,b,&tots,L)
       }
   }
   match (tally[d].get(&a),tally[d].get(&b)) {
       (None,None) => d,
       (_,_) => if d<(D-1) {
                    next(d+1,D,&tally,a,b,&tots,L-1)
                } else {
                    next(0,D,&tally,a,b,&tots,L-1)
                }
   }
}


fn sorted(input: &Input) -> Output {
    let M = input.es.len();
    let D = input.D;
    let K = input.K;
    let mut tally:Vec<HashMap<usize,usize>> = (0..D).map(|_| HashMap::new()).collect();
    let mut tots = vec![K; D];
    let mut ordered : Vec<(usize,&(usize,usize, i32))> = input.es.iter().enumerate().collect();
    ordered.sort_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap());
    let mut out= vec![0; M];
    for n in 0..M {
        let d = next(n%D,D,&tally,(ordered[n].1).0,(ordered[n].1).1,&tots,D);
        match tally[d].get(&(ordered[n].1).0) {
            Some(&j) => tally[d].insert((ordered[n].1).0,j+1),
            _ => tally[d].insert((ordered[n].1).0,1),
        };
        match tally[d].get(&(ordered[n].1).1) {
            Some(&j) => tally[d].insert((ordered[n].1).1,j+1),
            _ => tally[d].insert((ordered[n].1).1,1),
        };
        out[ordered[n].0]=1+d;
        tots[d] -= 1;
    }
    out
}

pub fn main() {
    let input = parse_input();
    let guess0 = sorted(&input);
    println!("{}", write(&guess0));
}
