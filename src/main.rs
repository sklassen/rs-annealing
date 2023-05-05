#![allow(non_snake_case, unused_macros)]

use proconio::input;
use rand::prelude::*;

const TIME_MAX: f64 = 2.0;

#[derive(Clone, Debug)]
pub struct Input {
    pub D: usize,
    pub cs: Vec<i32>,
    pub ss: Vec<Vec<i32>>,
}

fn parse_input() -> Input {
    input! {
        D: usize, 
        cs: [i32; 26],
        ss: [ [i32; 26]; D],
    }
    Input { D, cs, ss }
}

pub type Output = Vec<i32>;

fn write(output: &Output) -> String {
    let x: Vec<String> = output.iter().map( |&id| id.to_string()).collect();
    x.join(" ")
}

fn initial(d: usize) -> Vec<i32> {
    (0..d).map(|d| 1 + d as i32 % 26 as i32).collect()
}


//fn initial(d: usize) -> Vec<i32> {
//    let nums: Vec<i32> = (1..26).map(|x|(x,1)).collect();
//    let mut rng = rand::thread_rng();
//    let v: Vec<i32> = nums.choose_multiple_weighted(&mut rng, d, |x| x.1).unwrap().cloned().collect();
//
//    v
//}

fn score(input: &Input, choices: &Output) -> i32{
    let D = input.D;
    let K = 26;
    let mut visited = vec![0i32; K];
    let mut out=0i32;
    for d in 0..D {
        let c = choices[d] as usize - 1;
        visited[c]=d  as i32 + 1;
        let gain :i32 = input.ss[d][c];
        let loss :i32 = input.cs.iter().zip(visited.iter()).map(|(&a,&b)| a*((d as i32 + 1)-b)).sum();
        out +=  gain - loss;
    }
    out
}

fn neighbor(choices: &mut Vec<i32>, d: i32)  {
    let mut rng = rand::thread_rng();
    let nums: Vec<i32> = (1..26).collect();
    let days: Vec<i32> = (1..d).collect();
    let day = *days.choose(&mut rng).unwrap();
    let num = *nums.choose(&mut rng).unwrap();
    choices[day as usize] = num
}

pub fn main() {
    let start = std::time::Instant::now();
    let input = parse_input();
    let mut choices = initial(input.D);
    let mut prev_score = std::i32::MIN;
    let mut best_choice = choices.clone();
    let mut best_score = std::i32::MIN;
    let temperature = |d:f64| f64::powi(1. - d/TIME_MAX,2);
    let mut rng = rand::thread_rng();
    //eprintln!("choices {:?}", choices);
    loop {
        let end = start.elapsed();
        if end.as_secs_f64() > TIME_MAX * 0.99 {
           break;
        }
        let t = end.as_secs_f64();

        let T = temperature(t);
        let s = score(&input,&choices);
        //eprintln!("{} {} {}",i,T,s);
        if s > best_score {
            best_score = s;
            best_choice = choices.clone();
        } else {
            let y: f64 = rng.gen(); // generates a float between 0 and 1
            if  y>T {
                choices = best_choice.clone()
            } else {
                let grad = s as f64 / prev_score as f64 - 1.;

                if grad < 0.0 && grad.abs() > T {
                    choices = best_choice.clone()
                } 
            }
        }
        prev_score = s;
        neighbor(&mut choices,input.D as i32);
    }
    println!("{}", write(&best_choice));
    eprintln!("{}", best_score);
}
