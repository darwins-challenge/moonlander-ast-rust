use super::super::structure::Program;
use super::super::copy;
use super::mutation::Mutate;
use super::crossover;
use rand;
use rand::Rng;
use std::iter::Iterator;

pub struct Population {
    /// Collection of algorithms
    population: Vec<Program>,

    /// Collection of fitness scores
    pub scores: Vec<f64>
}

impl Population {
    /// Create a new population with an estimated size
    ///
    /// This does not create programs yet but simply allocates memory.
    pub fn new(n: usize) -> Population {
        Population {
            population: Vec::with_capacity(n),
            scores: vec![0.0; n]
        }
    }

    /// Add a single program to the population
    pub fn add(&mut self, program: Program) {
        self.population.push(program);
    }

    pub fn n(&self) -> usize {
        self.population.len()
    }

    /// Apply a scoring function to the entire population
    pub fn score<F>(&mut self, scoring_fn: F)
        where F: Fn(&Program) -> f64 {
        // FIXME: Parallelize?
        self.scores = self.population.iter().map(scoring_fn).collect();
    }

    /// Select a tournament winner from a tournament round of size n
    pub fn select_tournament_winner<R: rand::Rng>(&self, n: usize, rng: &mut R) -> &Program {
        self.population.get(self.select_tournament_winner_i(n, rng)).unwrap()
    }

    pub fn select_tournament_winner_i<R: rand::Rng>(&self, n: usize, rng: &mut R) -> usize {
        let candidate_indexes = rand::sample(rng, 0..self.n(), n);
        let (_, winner) = partial_max(candidate_indexes.into_iter().map(|i| (self.scores[i], i))).unwrap();
        winner
    }

    /// Return the best program from the population
    pub fn best(&self) -> (&Program, f64) {
        let indexes = 0..self.n();
        let (score, winner) = partial_max(indexes.into_iter().map(|i| (self.scores[i], i))).unwrap();
        (self.population.get(winner).unwrap(), score)
    }

    /// Produce a new population of the same size based off the current one
    pub fn evolve<R: rand::Rng>(&self, tournament_size: usize, reproduce_weight: u32, mutate_weight: u32, crossover_weight: u32, rng: &mut R) -> Population {
        let mut ret = Population::new(self.n());
        while ret.n() < self.n() {
            pick![
                reproduce_weight, ret.add(self.select_tournament_winner(tournament_size, rng).clone()),
                mutate_weight, ret.add(self.select_tournament_winner(tournament_size, rng).mutate(rng)),
                crossover_weight, {
                    if self.n() < 2 { continue; }

                    let (one, two) = self.pick_two(tournament_size, rng);

                    let (child1, child2) = crossover::cross_over(one, two, rng);
                    // We try to insert both children, but only if there's room in the population
                    ret.add(*child1);
                    if ret.n() < self.n() {
                        ret.add(*child2);
                    }
                }
            ];
        }
        ret
    }

    pub fn pick_two<R: rand::Rng>(&self, tournament_size: usize, rng: &mut R) -> (&Program, &Program) {
        loop {
            let one = self.select_tournament_winner(tournament_size, rng);
            let two = self.select_tournament_winner(tournament_size, rng);
            if !copy::ref_eq(one, two) {
                return (one, two);
            }
        }
    }
}

/// A max() function that only requires a partial ordering.
///
/// Necessary for floats because they don't implement a total ordering, something that the regular
/// Iterator::max() function needs.
fn partial_max<I: Iterator>(iter: I) -> Option<I::Item>
    where I::Item : PartialOrd {
    iter.fold(None, |ret, x| {
        match ret {
            None => Some(x),
            Some(ref y) if x > *y => Some(x),
            _ => ret
        }
    })
}

/// Generate a random population of size n
pub fn random_population(n: usize) -> Population {
    let mut ret = Population::new(n);
    for _ in 0..n {
        ret.add(rand::random());
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::visit::{Visitor, Visitable};
    use super::super::super::structure::*;
    use rand;

    // This makes the macros work (which expect stuff to be in ast::structure::...etc...)
    mod ast { pub use super::super::super::super::structure; }

    #[test]
    fn population_scoring() {
        let mut p = Population::new(10);
        p.add(skip!());
        p.add(left!());
        p.add(right!());
        p.add(thrust!());

        score_population(&mut p);

        assert_eq!(vec![0.0, 1.0, 2.0, 3.0], p.scores);
    }

    #[test]
    fn population_tournament() {
        let mut p = Population::new(10);
        p.add(skip!());
        p.add(left!());
        p.add(right!());
        p.add(thrust!());

        score_population(&mut p);

        let mut rng = rand::thread_rng();
        // This must always produce the single best one
        assert_eq!(3, p.select_tournament_winner_i(4, &mut rng));

        // This must produce one of the last 2 ones
        assert!(p.select_tournament_winner_i(3, &mut rng) >= 2);
    }

    fn score_population(p: &mut Population) {
        p.score(|x| { 
            let mut c = CommandCounter::new();
            x.visit(&mut c);
            c.value as f64
        });
    }

    /// A simple visitor that counts the enum values of commands and sensors
    struct CommandCounter {
        value: u32
    }

    impl CommandCounter {
        fn new() -> CommandCounter {
            CommandCounter { value: 0 }
        }
    }

    impl <'a> Visitor<'a> for CommandCounter {
        fn visit_program(&mut self, _: &'a Program) { }
        fn visit_condition(&mut self, _: &'a Condition) { }
        fn visit_expression(&mut self, _: &'a Expression) { }
        
        fn visit_command(&mut self, command: &'a Command) {
            self.value += command.clone() as u32;
        }

        fn visit_sensor(&mut self, sensor: &'a Sensor) {
            self.value += sensor.clone() as u32;
        }
    }
}
