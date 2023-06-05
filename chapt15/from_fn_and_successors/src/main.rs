use num::Complex;
use std::iter::successors;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex { re: 0.0, im: 0.0 };
    successors(Some(zero), |&z| { Some(z * z + c) })
        .take(limit)
        .enumerate()
        .find(|(_i, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| i)
}

fn fibonacci() -> impl Iterator<Item=usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}


fn main() {
    use rand::random; // In Cargo.toml dependencies: rand = "0.7"
    use std::iter::from_fn;

    // Generate the lengths of 1000 random line segments whose endpoints
    // are uniformly distributed across the interval [0, 1]. (This isn't a
    // distribution you're going to find in the `rand_distr` crate, but
    // it's easy to make yourself.)
    let lengths: Vec<f64> = 
        from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(1000)
        .collect();

    let its = lengths.into_iter();
    for item in its {
        println!("{:?}", item);
    }

    assert_eq!(fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]);

}
