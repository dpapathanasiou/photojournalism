use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

pub fn randomize(seed: u64, max_val: usize) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut x = (0..max_val).into_iter().map(|i| i).collect::<Vec<_>>();
    x.shuffle(&mut rng);
    x
}