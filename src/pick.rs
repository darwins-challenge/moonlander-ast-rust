extern crate rand;

/// The `pick` macro allows to randomly select one expression from a sequence of
/// expressions according to associated weights.
#[macro_export]
macro_rules! pick {
    ($( $weight: expr, $expression: expr),+) => {{
        let total = 0 $(+ $weight)+;
        let mut bound = 0;

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0, total);
        let result = $( if bound <= random_number && random_number < { bound += $weight; bound } {
            $expression
        } else )+ {
           panic!();
        };
        result
    }}
}

