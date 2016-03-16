#[macro_use]
extern crate ast;
extern crate rand;
extern crate rustc_serialize;

pub use rand::Rng;

use ast::structure::Program;
use ast::source::Source;

fn main() {
    let generated: Program = rand::random();
	  println!("{}", generated.source());
}
