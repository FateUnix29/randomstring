use rand::Rng;

#[allow(dead_code)]
pub fn randvec(length: usize, min: i64, max: i64) -> Vec<i64> {
    let mut resulting_vec: Vec<i64> = Vec::new();
    for _i in 0..length {
    let current_rand: i64 = rand::thread_rng().gen_range(min..max);
        resulting_vec.push(current_rand)
    }
    resulting_vec
}