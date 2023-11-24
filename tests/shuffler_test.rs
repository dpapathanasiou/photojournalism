use photojournalism::shuffler::randomize;

#[test]
fn same_seed_produces_same_shuffle_consistently() {
    let seed: u64 = 42;
    let max_val: usize = 1024;

    let a = randomize(seed, max_val);
    let b = randomize(seed, max_val);
    let c = randomize(seed, max_val);

    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}

#[test]
fn different_seed_produces_different_shuffles() {
    let seed_a: u64 = 42;
    let seed_b: u64 = 127;
    let max_val: usize = 2048;

    let a = randomize(seed_a, max_val);
    let b = randomize(seed_b, max_val);

    assert_ne!(a, b);
}
