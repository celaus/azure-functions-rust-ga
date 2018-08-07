#![feature(test, int_to_from_bytes, use_extern_macros, wasm_custom_section, wasm_import_module)]
extern crate rand;
extern crate rsgenetic;
extern crate test;
extern crate wasm_bindgen;

use rand::prelude::*;
use rand::prng::XorShiftRng;
use rand::rngs::OsRng;
use rsgenetic::pheno::*;
use rsgenetic::sim::select::*;
use rsgenetic::sim::seq::Simulator;
use rsgenetic::sim::*;
use rsgenetic::stats::StatsCollector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

const INDPB: f32 = 0.05f32;
const GENERATIONS: usize = 400;
const POPULATION_SIZE: usize = 100;

type City = (f32, f32);
type Tour = Vec<usize>;
type TourFitness = i32;

#[wasm_bindgen]
extern "C" {
    fn random() -> f32;
}

fn distance(p1: &City, p2: &City) -> f32 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

#[wasm_bindgen]
pub struct TSPSolution {
    citizen: Vec<City>,
    history: Vec<f32>,
}

struct FitnessHistoryEntry {
    pub min: TourFitness,
    pub max: TourFitness,
    pub avg: f32,
}

struct FitnessHistory {
    pub history: Vec<FitnessHistoryEntry>,
}

#[derive(Clone)]
struct TspTour {
    tour: Tour,
    cities: Rc<Vec<City>>,
    rng_cell: Rc<RefCell<XorShiftRng>>,
}

impl TspTour {
    pub fn new(tour: Tour, cities: Rc<Vec<City>>, rng_cell: Rc<RefCell<XorShiftRng>>) -> TspTour {
        TspTour {
            tour: tour,
            cities: cities,
            rng_cell: rng_cell,
        }
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
        // PMX crossover
        let s = &self.tour;
        let t = &other.tour;

        let mut rng = self.rng_cell.borrow_mut();
        let x1 = rng.gen_range(0, s.len() - 1);
        let x2 = rng.gen_range(x1, s.len());

        let mut offspring = vec![0; s.len()];
        let mut mapping: Vec<Option<usize>> = vec![None; s.len()];

        for i in x1..x2 {
            offspring[i] = t[i];
            mapping[t[i]] = Some(s[i]);
        }

        for i in 0..x1 {
            let mut o = mapping[s[i]];
            let mut last = None;
            while o.is_some() {
                last = o;
                o = mapping[o.unwrap()];
            }
            offspring[i] = if let Some(v) = last { v } else { s[i] };
        }

        for i in x2..s.len() {
            let mut o = mapping[s[i]];
            let mut last = None;
            while o.is_some() {
                last = o;
                o = mapping[o.unwrap()];
            }
            offspring[i] = if let Some(v) = last { v } else { s[i] };
        }

        TspTour {
            tour: offspring,
            cities: self.cities.clone(),
            rng_cell: self.rng_cell.clone(),
        }
    }

    fn mutate(&self) -> TspTour {
        let mut rng = self.rng_cell.borrow_mut();
        if rng.gen::<f32>() < INDPB {
            let mut mutated = self.tour.clone();
            rng.shuffle(&mut mutated);
            TspTour {
                tour: mutated,
                cities: self.cities.clone(),
                rng_cell: self.rng_cell.clone(),
            }
        } else {
            self.clone()
        }
    }
}

impl StatsCollector<TourFitness> for FitnessHistory {
    fn after_step(&mut self, pop_fitness: &[TourFitness]) {
        let mut sum: i64 = 0;
        let mut min: TourFitness = <i32>::max_value();
        let mut max = <i32>::min_value();
        for f in pop_fitness.into_iter() {
            sum += *f as i64;
            if -f < min {
                min = *f;
            }

            if -f > max {
                max = *f;
            }
        }
        self.history.push(FitnessHistoryEntry {
            min: min,
            max: max,
            avg: (sum as f64 / pop_fitness.len() as f64) as f32,
        })
    }
}

const N: usize = 16;
#[wasm_bindgen]
pub fn sovle_tsp(x: Vec<f32>, y: Vec<f32>) -> TSPSolution {
    let cities: Rc<Vec<City>> = Rc::new(x.into_iter().zip(y.into_iter()).collect());
    let city_indices: Vec<usize> = (0..cities.len()).collect();

    let mut population: Vec<TspTour> = Vec::with_capacity(POPULATION_SIZE);
    let mut rng = OsRng::new().unwrap();
    let mut seed: [u8; N] = [0; 16];

    for i in 0..N {
        seed[i] = (random() * 255f32).floor() as u8;
    }

    let mut rng = XorShiftRng::from_seed(seed);
    let sim_rng = Rc::new(RefCell::new(XorShiftRng::from_seed(seed)));

    for _ in 0..POPULATION_SIZE {
        let mut pheno = city_indices.clone();
        rng.shuffle(&mut pheno);
        population.push(TspTour::new(pheno, cities.clone(), sim_rng.clone()));
    }

    let history_collector = Rc::new(RefCell::new(FitnessHistory {
        history: Vec::with_capacity(GENERATIONS),
    }));

    #[allow(deprecated)]
    let mut s: Simulator<_, _, FitnessHistory> =
        Simulator::builder_with_stats(&mut population, Some(history_collector.clone()))
            .set_selector(Box::new(MaximizeSelector::new(10)))
            .set_max_iters(GENERATIONS as u64)
            .build();
    s.run();
    let result = s.get().unwrap();

    let avg_history: Vec<f32> = history_collector.borrow().history.iter().map(|h| h.avg).collect();

    TSPSolution {
        citizen: result.tour.iter().map(|t| cities[*t]).collect(),
        history: avg_history,
    }
    // result.tour.clone().into_iter().map(|r| r as i32).collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;
    use std::collections::BTreeSet;
    use std::iter::FromIterator;
    use test::Bencher;

    #[test]
    fn test_nothing() {
        let mut rng = OsRng::new().unwrap();

        let x: Vec<f32> = (0..20).map(|_| rng.gen_range(0, 590) as f32).collect();
        let y: Vec<f32> = (0..20).map(|_| rng.gen_range(0, 590) as f32).collect();

        let cities: Rc<Vec<City>> = Rc::new(x.into_iter().zip(y.into_iter()).collect());
        let city_indices: Vec<usize> = (0..cities.len()).collect();
        let mut population: Vec<TspTour> = Vec::with_capacity(POPULATION_SIZE);
        let seed: [u8; 16] = rng.gen::<[u8; 16]>();

        let sim_rng = Rc::new(RefCell::new(XorShiftRng::from_seed(seed)));

        for _ in 0..POPULATION_SIZE {
            let mut pheno = city_indices.clone();
            rng.shuffle(&mut pheno);
            population.push(TspTour::new(pheno, cities.clone(), sim_rng.clone()));
        }

        let history_collector = FitnessHistory {
            history: Vec::with_capacity(GENERATIONS),
        };

        #[allow(deprecated)]
        let mut s: Simulator<_, _, FitnessHistory> =
            Simulator::builder_with_stats(
                &mut population,
                Some(Rc::new(RefCell::new(history_collector))),
            ).set_selector(Box::new(MaximizeSelector::new(10)))
                .set_max_iters(GENERATIONS as u64)
                .build();
        s.run();
        let result = BTreeSet::from_iter(s.get().unwrap().tour.iter());
        assert_eq!(result.len(), cities.len());
    }

    #[bench]
    fn bench_monte_carlo_pi(b: &mut Bencher) {
        //b.iter(|| monte_carlo_pi(1000));
    }
}
