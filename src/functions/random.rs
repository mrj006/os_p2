use rand::Rng;

pub fn random(count: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..count).map(|_| rng.random_range(min..=max)).collect()
}
