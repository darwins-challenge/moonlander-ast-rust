use std::cmp::{Ordering};
use super::super::copy;
use super::super::source::Source;
use super::super::num::partial_max;
use super::super::visit::Visitable;
use super::super::simplify::Simplify;
use super::super::structure::Number;
use super::super::serialize::GameTrace;
use super::mutation;
use super::crossover;
use rand;
use rand::{Rng, Rand};
use std::iter::Iterator;
use std::ops::Add;

/// A population with the root of the indicated type
pub struct Population<P: Rand+Clone+Source> {
    /// Collection of algorithms
    pub population: Vec<P>,

    /// Generation index of this population
    pub generation: u32,

    /// Collection of fitness scores
    pub scores: Vec<ScoreCard>
}

impl <P: Rand+Clone+Source> Population<P> {
    /// Create a new population with an estimated size
    ///
    /// This does not create programs yet but simply allocates memory.
    pub fn new(n: usize, generation: u32) -> Population<P> {
        Population {
            population: Vec::with_capacity(n),
            scores: Vec::with_capacity(n),
            generation: generation
        }
    }

    /// Add a single program to the population
    pub fn add(&mut self, program: P) {
        self.population.push(program);
    }

    pub fn n(&self) -> usize {
        self.population.len()
    }

    /// Apply a scoring function to the entire population
    pub fn score<F>(&mut self, scoring_fn: F)
        where F: FnMut(&P) -> ScoreCard
    {
        // FIXME: Parallelize?
        self.scores = self.population.iter().map(scoring_fn).collect();
    }

    /// Select a tournament winner from a tournament round of size n
    pub fn select_tournament_winner<R: rand::Rng>(&self, n: usize, rng: &mut R) -> &P {
        self.population.get(self.select_tournament_winner_i(n, rng)).unwrap()
        //unsafe { self.population.get_unchecked(self.select_tournament_winner_i(n, rng)) }
    }

    pub fn select_tournament_winner_i<R: rand::Rng>(&self, n: usize, rng: &mut R) -> usize {
        // Generate N random indexes. Slightly faster than rand::sample(), don't care about
        // the inaccuracy introduced by sampling with replacement.
        let count = self.n();
        let candidate_indexes = (0..n).map(|_| rng.next_u64() as usize % count);

        //let candidate_indexes = rand::sample(rng, 0..self.n(), n);
        let (_, winner) = partial_max(candidate_indexes.map(|i| (&self.scores[i], i))).unwrap();
        winner
    }

    /// Return the best program from the population
    pub fn winner(&self) -> CreatureScore<P> {
        let indexes = 0..self.n();
        let (score, winner_i) = partial_max(indexes.into_iter().map(|i| (&self.scores[i], i))).unwrap();
        CreatureScore::new(self.population.get(winner_i).unwrap().clone(), score.clone())
    }

    /// Produce a new population of the same size based off the current one
    pub fn evolve<'a, R: rand::Rng>(&'a self, tournament_size: usize, reproduce_weight: u32, mutate_weight: u32, crossover_weight: u32, rng: &mut R) -> Population<P> 
        where P: Visitable<'a>+copy::Copyable // Additional bounds for crossover
    {
        let mut ret = Self::new(self.n(), self.generation + 1);
        while ret.n() < self.n() {
            pick![
                reproduce_weight, {
                    let winner = self.select_tournament_winner(tournament_size, rng);
                    debug!("Reproduce: {}", winner.source());
                    ret.add(winner.clone());
                },
                mutate_weight, {
                    let winner = self.select_tournament_winner(tournament_size, rng);
                    let mutation = mutation::mutate(winner, rng);
                    debug!("Mutation: {} into {}", winner.source(), mutation.source());
                    ret.add(mutation);
                },
                crossover_weight, {
                    if self.n() < 2 { continue; }

                    let (one, two) = self.pick_two(tournament_size, rng);

                    let (child1, child2) = crossover::cross_over(one, two, rng);

                    debug!("Crossover: {} & {} into {} & {}", one.source(), two.source(), child1.source(), child2.source());

                    // We try to insert both children, but only if there's room in the population
                    ret.add(child1);
                    if ret.n() < self.n() {
                        ret.add(child2);
                    }
                }
            ];
        }
        ret
    }

    pub fn pick_two<R: rand::Rng>(&self, tournament_size: usize, rng: &mut R) -> (&P, &P) {
        loop {
            let one = self.select_tournament_winner(tournament_size, rng);
            let two = self.select_tournament_winner(tournament_size, rng);
            if !copy::ref_eq(one, two) {
                return (one, two);
            }
        }
    }
}

pub type Scores = Vec<(&'static str, Number)>;

/// Immutable tagged list of scores
#[derive(Clone)]
pub struct ScoreCard(Scores, Number, GameTrace);

impl ScoreCard {
    pub fn new(scores: Scores, trace: GameTrace) -> ScoreCard {
        let sum = scores.iter().map(|&(_, x)| x).fold(0.0, Add::add);
        ScoreCard(scores, sum, trace)
    }

    pub fn add(self, scores: Scores) -> ScoreCard {
        let mut xs = self.0;
        xs.extend(scores);
        ScoreCard::new(xs, self.2)
        // Which one is faster? 
        /*
        let all_scores = self.0.into_iter().chain(scores.into_iter()).collect();
        ScoreCard::new(all_scores, self.2)
        */
    }

    pub fn scores(&self) -> &Scores {
        &self.0
    }

    pub fn total_score(&self) -> Number {
        self.1
    }

    pub fn trace(&self) -> &GameTrace {
        &self.2
    }
}

impl PartialEq for ScoreCard {
    fn eq(&self, other: &Self) -> bool {
        return self.1.eq(&other.1);
    }
}

impl Eq for ScoreCard {
}

impl PartialOrd for ScoreCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.1.partial_cmp(&other.1);
    }
}

pub struct CreatureScore<P> {
    pub program: P,
    pub score: ScoreCard
}

impl <P> CreatureScore<P> {
    pub fn new(program: P, score: ScoreCard) -> CreatureScore<P> {
        CreatureScore { program: program, score: score }
    }
}

/// Generate a random population of size n
pub fn random_population<P: Rand+Clone+Source>(n: usize) -> Population<P> {
    let mut ret = Population::new(n, 0);
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
    use super::super::super::serialize::*;
    use rand;

    // This makes the macros work (which expect stuff to be in ast::structure::...etc...)
    mod ast { pub use super::super::super::super::structure; }

    #[test]
    fn population_scoring() {
        let mut p = Population::new(10, 1);
        p.add(skip!());
        p.add(left!());
        p.add(right!());
        p.add(thrust!());

        score_population(&mut p);

        assert_eq!(vec![0.0, 1.0, 2.0, 3.0],
                   p.scores.iter().map(|s| s.total_score()).collect::<Vec<Number>>());
    }

    #[test]
    fn population_tournament() {
        let mut p = Population::new(10, 1);
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

    fn score_population(p: &mut Population<Program>) {
        p.score(|x| { 
            let mut c = CommandCounter::new();
            x.visit(&mut c);
            ScoreCard::new(vec![
                           ("score", c.value as Number)
                           ], GameTrace::new())
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

pub struct OptimumKeeper<P> {
    best_program: Option<P>,
    best_score: Option<ScoreCard>,
    best_generation: u32
}

impl <P: Simplify+Clone> OptimumKeeper<P> {
    pub fn new() -> OptimumKeeper<P> {
        OptimumKeeper { best_program: None, best_score: None, best_generation: 0 }
    }

    pub fn improved(&mut self, program: &P, score: &ScoreCard, generation: u32) -> bool {
        if self.best_score.is_none() || score > self.best_score.as_ref().unwrap() {
            self.best_program = Some(program.simplify());
            self.best_score = Some(score.clone());
            self.best_generation = generation;
            true
        } else {
            false
        }
    }
}
