//! This crate explores [genetic
//! programming](https://en.wikipedia.org/wiki/Genetic_programming). Genetic
//! programming is
//!
//! > a technique whereby computer programs are encoded as a set of genes that
//! > are then modified (evolved) using an evolutionary algorithm.
//!
//! The problem we are trying to tackle with genetic programming is that of
//! landing a [moon lander](https://en.wikipedia.org/wiki/Lunar_lander). I.e.
//! land a moon lander safely on the surface of a moon without user
//! intervention. A nice interactive game that gives a feel for the problem can be
//! found [here](http://moonlander.seb.ly/).

extern crate rand;
extern crate rustc_serialize;

#[macro_use]
pub mod pick;

#[macro_use]
pub mod macros;

pub mod copy;
pub mod darwin;
pub mod data;
pub mod random;
pub mod serialize;
pub mod simplify;
pub mod simulation;
pub mod source;
pub mod structure;
pub mod visit;
