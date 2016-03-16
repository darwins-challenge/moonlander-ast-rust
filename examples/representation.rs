#[macro_use]
extern crate ast;
extern crate rand;
extern crate rustc_serialize;

pub use rand::Rng;

use ast::structure::Program;
use ast::source::Source;

fn main() {
	  let program: Program = iff!(less!(vx!(),multiply!(constant!(2.0000),constant!(3.0000))),left!(),right!());

	  println!("{}", program.source());
}
