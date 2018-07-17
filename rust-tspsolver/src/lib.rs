#![feature(test, int_to_from_bytes, proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate test;
extern crate rsgenetic;
extern crate rand;

use wasm_bindgen::prelude::*;
use rsgenetic::sim::*;
use rsgenetic::sim::seq2::Simulator;
use rsgenetic::sim::select::*;
use rsgenetic::pheno::*;
use rand::prng::XorShiftRng;
use rand::{FromEntropy,Rng};
use std::rc::Rc;

const INDPB: f32 = 0.05f32;
const GENERATIONS: u64 = 400;
const POPULATION_SIZE: usize = 100;

type City = (f32, f32);
type Tour = Vec<usize>;
type TourFitness = i32;

fn distance(p1: &City, p2: &City) -> f32 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

#[derive(Clone)]
struct TspTour {
    tour: Tour,
    cities: Rc<Vec<City>>
}

impl TspTour {
    pub fn new(tour: Tour, cities: Rc<Vec<City>>) -> TspTour {
        TspTour { tour: tour, cities: cities }
    }
}

impl Phenotype<TourFitness> for TspTour {
    fn fitness(&self) -> TourFitness {
        let tour_cities: Vec<&City> = self.tour.iter().map(|t| &self.cities[*t]).collect();
        let mut fitness = 0f32;
        for i in 1..tour_cities.len() {
            fitness += distance(tour_cities[i], tour_cities[i - 1]);
        }
        -(fitness.round() as i32)
    }

    fn crossover(&self, other: &TspTour) -> TspTour {
        // 2-way crossover
        let mut rng = XorShiftRng::new_unseeded();
        let crossover_indices = (
            rng.gen_range::<usize>(0, self.tour.len()),
            rng.gen_range::<usize>(0, self.tour.len() - 1),
        );
        let mut crossed_over: Tour = vec![0; self.tour.len()];
        for i in 0..crossover_indices.0 {
            crossed_over[i] = self.tour[i];
        }
        for i in crossover_indices.0..crossover_indices.1 {
            crossed_over[i] = other.tour[i];
        }
        for i in crossover_indices.1..self.tour.len() {
            crossed_over[i] = self.tour[i];
        }
        TspTour {
            tour: crossed_over,
            cities: self.cities.clone()
        }
    }

    fn mutate(&self) -> TspTour {
        let mut rng = XorShiftRng::new_unseeded();
        if rng.gen::<f32>() < INDPB {
            let mut mutated = self.tour.clone();
            rng.shuffle(&mut mutated);
            TspTour{
                tour: mutated,
                cities: self.cities.clone()
            }
        }
        else {
            self.clone()
        }
    }
}


#[wasm_bindgen]
pub fn sovle_tsp(x: Vec<f32>, y: Vec<f32>) -> Vec<i32> {
    let cities: Rc<Vec<City>> = Rc::new(x.into_iter().zip(y.into_iter()).collect());
    let city_indices: Vec<usize> = (0..cities.len()).collect();

    let mut population: Vec<TspTour> = Vec::with_capacity(POPULATION_SIZE);
    let mut rng = XorShiftRng::new_unseeded();
    for _ in 0..POPULATION_SIZE {
        let mut pheno = city_indices.clone();
        rng.shuffle(&mut pheno);
        population.push(TspTour::new(pheno, cities.clone()));
    }
    
    #[allow(deprecated)]
    let mut s = Simulator::builder(&mut population)
        .set_selector(Box::new(MaximizeSelector::new(10)))
        .set_max_iters(GENERATIONS)
        .build();
    s.run();
    let result = s.get().unwrap();
    //let time = s.time();
    //println!("Execution time: {} ns.", time.unwrap());
    println!(
        "Result: {:?} | Fitness: {}.",
        result.tour,
        result.fitness()
    );
    result.tour.clone().into_iter().map(|r| r as i32).collect()
}


#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;
    use test::Bencher;

    #[test]
    fn test_nothing() {
        const N: usize = 30;
        let mut rng = rand::thread_rng();
        let mut x = Vec::with_capacity(N);
        let mut y = Vec::with_capacity(N);

        for _ in 0..N {
            x.push(rng.gen_range::<f32>(10f32, 690f32));
            y.push(rng.gen_range::<f32>(10f32, 580f32));
        }
        assert_eq!(x.len(), N);
        assert_eq!(y.len(), N);
        let result = sovle_tsp(x, y);
        assert_eq!(result,vec![]);
    }

    #[bench]
    fn bench_monte_carlo_pi(b: &mut Bencher) {
        //b.iter(|| monte_carlo_pi(1000));
    }
}