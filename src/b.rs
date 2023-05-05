#![allow(non_snake_case, unused_macros)]

use proconio::input;

#[derive(Clone, Debug)]
pub struct Input {
    pub D: usize,
    pub cs: Vec<i32>,
    pub ss: Vec<Vec<i32>>,
    pub ts: Vec<i32>,
}

fn parse_input() -> Input {
    input! {
        D: usize, 
        cs: [i32; 26],
        ss: [ [i32; 26]; D],
        ts: [i32; D],
    }
    Input { D, cs, ss,  ts }
}

pub type Output = Vec<i32>;

fn write(output: &Output) -> String {
    let x: Vec<String> = output.iter().map( |&id| id.to_string()).collect();
    x.join(" ")
}

fn score(input: &Input, choices: &Output) -> Vec<i32>{
    let D = input.D;
    let K = 26;
    let mut visited = vec![0i32; K];
    let mut out= vec![0i32; D];
    let mut prev = 0i32;
    for d in 0..D {
        let c = choices[d] as usize - 1;
        visited[c]=d  as i32 + 1;
        let gain :i32 = input.ss[d][c];
        let loss :i32 = input.cs.iter().zip(visited.iter()).map(|(&a,&b)| a*((d as i32 + 1)-b)).sum();
        out[d] = prev + gain - loss;
        prev = out[d];
    }
    out
}

pub fn main() {
    let input = parse_input();
    let s = score(&input,&input.ts);
    println!("{}", write(&s));
}
