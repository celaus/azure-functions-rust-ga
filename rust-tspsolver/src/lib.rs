#![feature(test, int_to_from_bytes, use_extern_macros, wasm_custom_section, wasm_import_module)]
extern crate rand;
extern crate rsgenetic;
extern crate test;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use rand::prelude::*;
use rand::prng::XorShiftRng;
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
const MUTPROB: f32 = 0.2f32;

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

#[derive(Serialize, Deserialize, Debug)]
struct TSPSolution {
    pub citizen: Tour,
    pub history: Vec<f32>,
}

struct FitnessHistoryEntry {
    pub min: TourFitness,
    pub max: TourFitness,
    pub avg: f32,
}

struct FitnessHistory {
    pub history: Vec<FitnessHistoryEntry>,
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

#[derive(Clone, Debug)]
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
        if rng.gen::<f32>() < MUTPROB {
            let mut mutated: Tour = self.tour.clone();
            for i in 0..mutated.len() {
                if rng.gen::<f32>() < INDPB {
                    let mut swap_idx = rng.gen_range(0, mutated.len() - 2);
                    if swap_idx >= i {
                        swap_idx += 1;
                    }
                    let tmp = mutated[i];
                    mutated[i] = mutated[swap_idx];
                    mutated[swap_idx] = tmp;
                }
            }
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

const N: usize = 16;
fn create_random_seed() -> [u8; N] {
    let mut seed = [0u8; N];

    for i in 0..N {
        seed[i] = (random() * 255f32).floor() as u8;
    }
    seed
}

#[wasm_bindgen]
pub fn sovle_tsp(x: Vec<f32>, y: Vec<f32>, generations: usize, population_size: usize) -> String {
    let cities: Rc<Vec<City>> = Rc::new(x.into_iter().zip(y.into_iter()).collect());
    let city_indices: Vec<usize> = (0..cities.len()).collect();

    let mut population: Vec<TspTour> = Vec::with_capacity(population_size);

    let mut rng = XorShiftRng::from_seed(create_random_seed());
    let sim_rng = Rc::new(RefCell::new(XorShiftRng::from_seed(create_random_seed())));

    for _ in 0..population_size {
        let mut pheno = city_indices.clone();
        rng.shuffle(&mut pheno);
        population.push(TspTour::new(pheno, cities.clone(), sim_rng.clone()));
    }

    let history_collector = Rc::new(RefCell::new(FitnessHistory {
        history: Vec::with_capacity(generations),
    }));

    #[allow(deprecated)]
    let mut s: Simulator<_, _, FitnessHistory> =
        Simulator::builder_with_stats(&mut population, Some(history_collector.clone()))
            .set_selector(Box::new(MaximizeSelector::new((population_size / 2) - 2)))
            .set_max_iters(generations as u64)
            .build();
    s.run();

    let result = s.get().unwrap();

    let avg_history: Vec<f32> = history_collector
        .borrow()
        .history
        .iter()
        .map(|h| h.avg)
        .collect();

    serde_json::to_string(&TSPSolution {
        citizen: result.tour.clone(),
        history: avg_history,
    }).unwrap()
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
        let mut rng = thread_rng();
        let mut rng2 = thread_rng();
        
        let population_size = 100;
        let generations = 400;
        let mut rng = XorShiftRng::from_rng(rng).unwrap();


        let x: Vec<f32> = (0..20).map(|_| rng.gen_range(0, 590) as f32).collect();
        let y: Vec<f32> = (0..20).map(|_| rng.gen_range(0, 590) as f32).collect();

        let cities: Rc<Vec<City>> = Rc::new(x.into_iter().zip(y.into_iter()).collect());
        let city_indices: Vec<usize> = (0..cities.len()).collect();
        let mut population: Vec<TspTour> = Vec::with_capacity(population_size);
        let seed: [u8; 16] = rng.gen::<[u8; 16]>();

        let sim_rng = Rc::new(RefCell::new(XorShiftRng::from_rng(rng2).unwrap()));

        for _ in 0..population_size {
            let mut pheno = city_indices.clone();
            rng.shuffle(&mut pheno);
            population.push(TspTour::new(pheno, cities.clone(), sim_rng.clone()));
        }

        let history_collector = Rc::new(RefCell::new(FitnessHistory {
            history: Vec::with_capacity(generations),
        }));

        #[allow(deprecated)]
        let mut s: Simulator<_, _, FitnessHistory> =
            Simulator::builder_with_stats(&mut population, Some(history_collector.clone()))
                .set_selector(Box::new(MaximizeSelector::new(10)))
                .set_max_iters(generations as u64)
                .build();
        s.run();

        let result = s.get().unwrap();

        let avg_history: Vec<f32> = history_collector
            .borrow()
            .history
            .iter()
            .map(|h| h.avg)
            .collect();

        assert_eq!(result.tour.len(), cities.len());
    }

    #[bench]
    fn bench_monte_carlo_pi(b: &mut Bencher) {
        //b.iter(|| monte_carlo_pi(1000));
    }
}
