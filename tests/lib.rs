#[macro_use]
extern crate ast;

use ast::source::Source;

#[test]
fn iff_macro_should_work(){
    assert_eq!("iff!(T!(),skip!(),thrust!())", iff!(T!(), skip!(), thrust!()).source());
}

#[test]
fn t_macro_should_work(){
    assert_eq!("T!()", T!().source());
}

#[test]
fn f_macro_should_work(){
    assert_eq!("F!()", F!().source());
}

#[test]
fn not_macro_should_work(){
    assert_eq!("not!(F!())", not!(F!()).source());
}

#[test]
fn or_macro_should_work(){
    assert_eq!("or!(T!(),F!())", or!(T!(),F!()).source());
}

#[test]
fn and_macro_should_work(){
    assert_eq!("and!(T!(),F!())", and!(T!(),F!()).source());
}

#[test]
fn less_macro_should_work(){
    assert_eq!("less!(constant!(1.0000),constant!(2.0000))", less!(constant!(1.0),constant!(2.0)).source());
}

#[test]
fn less_equal_macro_should_work(){
    assert_eq!("less_equal!(constant!(1.0000),constant!(2.0000))", less_equal!(constant!(1.0),constant!(2.0)).source());
}

#[test]
fn equal_macro_should_work(){
    assert_eq!("equal!(constant!(1.0000),constant!(2.0000))", equal!(constant!(1.0),constant!(2.0)).source());
}

#[test]
fn greater_equal_macro_should_work(){
    assert_eq!("greater_equal!(constant!(1.0000),constant!(2.0000))", greater_equal!(constant!(1.0),constant!(2.0)).source());
}

#[test]
fn greater_macro_should_work(){
    assert_eq!("greater!(constant!(1.0000),constant!(2.0000))", greater!(constant!(1.0),constant!(2.0)).source());
}

#[test]
fn constant_macro_should_work(){
    assert_eq!("constant!(1.0000)", constant!(1.0).source());
}

#[test]
fn x_macro_should_work(){
    assert_eq!("x!()", x!().source());
}

#[test]
fn y_macro_should_work(){
    assert_eq!("y!()", y!().source());
}

#[test]
fn vx_macro_should_work(){
    assert_eq!("vx!()", vx!().source());
}

#[test]
fn vy_macro_should_work(){
    assert_eq!("vy!()", vy!().source());
}

#[test]
fn o_macro_should_work(){
    assert_eq!("o!()", o!().source());
}

#[test]
fn w_macro_should_work(){
    assert_eq!("w!()", w!().source());
}

#[test]
fn plus_macro_should_work(){
    assert_eq!("plus!(constant!(1.0000),constant!(2.0000))", plus!(constant!(1.0), constant!(2.0)).source());
}

#[test]
fn minus_macro_should_work(){
    assert_eq!("minus!(constant!(1.0000),constant!(2.0000))", minus!(constant!(1.0), constant!(2.0)).source());
}

#[test]
fn multiply_macro_should_work(){
    assert_eq!("multiply!(constant!(1.0000),constant!(2.0000))", multiply!(constant!(1.0), constant!(2.0)).source());
}

#[test]
fn divide_macro_should_work(){
    assert_eq!("divide!(constant!(1.0000),constant!(2.0000))", divide!(constant!(1.0), constant!(2.0)).source());
}

#[test]
fn skip_macro_should_work(){
    assert_eq!("skip!()", skip!().source());
}

#[test]
fn left_macro_should_work(){
    assert_eq!("left!()", left!().source());
}

#[test]
fn right_macro_should_work(){
    assert_eq!("right!()", right!().source());
}

#[test]
fn thrust_macro_should_work(){
    assert_eq!("thrust!()", thrust!().source());
}
