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


//fn _initial(d: usize) -> Vec<i32> {
//    let nums: Vec<(i32,i32)> = (1..26).map(|x|(x,1)).cycle().take(365).collect();
//
//    let mut rng = rand::thread_rng();
//    let v: Vec<i32> = nums.choose_multiple_weighted(&mut rng, d, |item| item.1).unwrap().map(|x|x.0).collect::<Vec<_>>();
//
//    v
//}

// Returns the total score and the lowest score
fn score(input: &Input, choices: &Output) -> i32 {
    let D = input.D;
    let mut visited = vec![0i32; 26];
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

fn neighbor(rng: &mut ThreadRng, choices: &mut Vec<i32>, d:i32)  {
    let days: Vec<i32> = (0..d-1).collect();
    loop {
        let day_a = *days.choose(rng).unwrap() as usize;
        let remain: Vec<i32> = (day_a as i32..d).collect();
        let day_b = *remain.choose(rng).unwrap() as usize;
        let a = choices[day_a];
        let b = choices[day_b];
        if a!=b {
            choices[day_a] = b;
            choices[day_b] = a;
            break
        }
    }
}

pub fn main() {
    let start = std::time::Instant::now();
    let input = parse_input();
    let mut choices = initial(input.D);
    let mut prev_score = std::i32::MIN;
    let mut best_choice = choices.clone();
    let mut best_score = std::i32::MIN;
    //let temperature = |d:f64| f64::powi(1. - d/TIME_MAX,2);
    let temperature = |d:f64| f64::exp(-TIME_MAX/d);
    let mut rng = rand::thread_rng();
    //let mut rng = rand_pcg::Pcg64Mcg::new(890482);
    //eprintln!("choices {:?}", choices);
    //let s = score(&input,&choices);
    //eprintln!("{} ",s);
    let mut T=1.0f64;
    let mut cnt =0i32;
    loop {
        cnt += 1;
        if cnt % 1000 == 0 {

            let end = start.elapsed();
            if end.as_secs_f64() > TIME_MAX * 0.99 {
                break;
            }
            let t = end.as_secs_f64();

            T = temperature(t);
        }
        let s= score(&input,&choices);
        //eprintln!("{} {} {}",i,T,s);
        if s > best_score {
            best_score = s;
            best_choice = choices.clone();
        } else {
            let t: f64 = rng.gen(); // generates a float between 0 and 1
            if  t>T {
                choices = best_choice.clone()
            } else {
                let grad = s as f64 / prev_score as f64 - 1.;

                if grad < 0.0 && grad.abs() < T*5. {
                    choices = best_choice.clone()
                } 
            }
        }
        prev_score = s;
        neighbor(&mut rng, &mut choices,input.D as i32);
    }
    println!("{}", write(&best_choice));
    eprintln!("{}", best_score);
}
