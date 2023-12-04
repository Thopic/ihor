mod utils;
mod parser;
use utils::Dna;
use parser::{ParserMarginals, ParserParams};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let s: Dna = Dna::from_string("ATCGTCTA");
    println!("{}", Dna::to_string(&s));


    let mut pm: ParserMarginals = Default::default();
    let res = pm.parse(Path::new("models/model_marginals.txt"));
    // println!("{:?}", pm);
    // println!("{:?}", res);

    let mut pm2: ParserParams = Default::default();
    let res2 = pm2.parse(Path::new("models/model_parms.txt"));
    println!("{:?}", pm2);
    println!("{:?}", res2);


}

//	let rng = SmallRng::seed_from_u64(seed);