/// Check whether `x` is prime
pub fn is_prime(x: i32) -> bool {
    let last = (x as f32).sqrt() as i32 + 1;
    return x > 1 && (2..last).all(|d| x % d != 0);
}