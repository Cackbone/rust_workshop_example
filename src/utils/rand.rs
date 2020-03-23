use rand::{ thread_rng, Rng };


pub fn random_bool(p: f64) -> bool {
    let mut rng = thread_rng();

    rng.gen_bool(p)
}
