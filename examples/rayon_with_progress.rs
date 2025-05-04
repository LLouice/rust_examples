use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    println!("parallel progress");
    let mut rng = rand::rng();
    let sz: usize = rng.random_range(1000..5000);

    let v = 0..sz;
    let v2: Vec<_> = v
        .into_par_iter()
        .progress_count(sz as u64)
        .map(|i| {
            let mut rng = rand::rng();
            let ms = rng.random_range(1..=10);
            std::thread::sleep(std::time::Duration::from_millis(ms));
            i + 1
        })
        .collect();
    let idx = rng.random_range(0..sz);
    assert_eq!(v2[idx], idx + 1);
    println!("end");
}
