#![allow(non_snake_case)]
use rand::Rng;
use rand::thread_rng;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};

fn AfficheEmetteursMaisons(Maison: Vec<(f64,f64)>,Emetteur: Vec<(f64,f64)>,rayon: f32){
    let s1: Plot = Plot::new(Maison).point_style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#DD3355"),
    );
    let s2: Plot = Plot::new(Emetteur).point_style(
        PointStyle::new() // uses the default marker
            .colour("#35C788")
            .size(rayon),
    );

    let v = ContinuousView::new()
        .add(s2)
        .add(s1)
        .x_range(0., 1000.)
        .y_range(0., 1000.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    Page::single(&v).save("scatter.svg").unwrap();
}

fn choixEmetteurGlouton(Maison:&Vec<(f64,f64)>)-> Vec<(f64,f64)>{
	return Maison.clone();
}

fn GenererMaisons(Maison:&mut Vec<(f64,f64)>, n:u64){
	let mut rng = thread_rng();
	for _ in 0..n{
		let x: f64 = rng.gen_range(1.0..1000.0);
		let y: f64 = rng.gen_range(1.0..1000.0);
		Maison.push((x, y));
	}
}

fn Couvre(Maison:&Vec<(f64,f64)>,i:usize,j:usize,rayon:f32)->bool{
	((Maison[i].0-Maison[j].0).powi(2)+(Maison[i].1-Maison[j].1).powi(2)).sqrt()< rayon.into()
}

fn choixMaison(Maison:&mut Vec<(f64,f64)>, MaisonRestantes:Vec<bool>, rayon:f32){
	for i1 in 0..Maison.len() {
		for i2 in 0..MaisonRestantes.len(){
			if !MaisonRestantes[i2] && Couvre(&Maison, i1, i2, rayon) {}
		}
	}
}

fn main() {
    let rayon=120_f32;
    let n=100;
    let mut Maison = vec![(3.0, 2.3)];
	
	GenererMaisons(&mut Maison,n);
	let Emetteur=choixEmetteurGlouton(&Maison);
	println!("L'algo glouton place {} émetteurs", Emetteur.len());
	AfficheEmetteursMaisons(Maison,Emetteur,rayon);
}

