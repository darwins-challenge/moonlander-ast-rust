#[macro_use]
extern crate ast;
extern crate rand;
extern crate rustc_serialize;

pub use rand::Rng;
use rustc_serialize::json;

use ast::structure::Program;

fn main() {
	  let program: Program = iff!(less!(vy!(),multiply!(constant!(2.0000),constant!(3.0000))),left!(),right!());

	  println!("{}", json::encode(&program).unwrap());
}
