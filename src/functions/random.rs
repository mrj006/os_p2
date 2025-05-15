use rand::Rng;

pub fn random(count: usize, min: i32, max: i32) -> Result<Vec<i32>, String> {
    // validación directa con Result
    (min <= max)
        .then_some(())
        .ok_or("min must be less than or equal to max".to_string())?;

    let mut rng = rand::rng();
    Ok((0..count).map(|_| rng.random_range(min..=max)).collect())
}
