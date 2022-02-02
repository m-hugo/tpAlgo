#![allow(non_snake_case)]
use std::io::stdin;
use rand::Rng;
use rand::thread_rng;

pub fn TriBulles(arr: &mut Vec<(u64, u64)>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j].0 + arr[j].1 > arr[j + 1].0 + arr[j + 1].1 {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn CoursAuHasard(n: u64) -> Vec<(u64, u64)>{
    let mut Cours=vec![];
    let mut rng = thread_rng();
    for _ in 0..n{
        Cours.push((rng.gen_range(1..90), rng.gen_range(1..10)));
    }
    Cours
}
fn main() {
	let mut s=String::new();
	stdin().read_line(&mut s).expect("Erreur 1");
	let n:u64 = s.trim().parse().expect("Entrez un nombre naturel");
	 let mut Cours=CoursAuHasard(n);
	 TriBulles(&mut Cours);
    println!("{:?}", Cours);
}
