pub fn fibonacci(n: u128) -> u128 {
    if !(n != 0 && n != 1) {
        return n;
    }

    let mut previous: u128 = 0;
    let mut current:u128 = 1;

    for _ in 0..(n-1) {
        let new = previous + current;
        previous = current;
        current = new;
    }

    return current;
}
