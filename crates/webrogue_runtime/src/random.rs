pub fn random_ctx() -> Box<dyn cap_rand::RngCore + Send + Sync> {
    let mut rng = cap_rand::thread_rng(cap_rand::ambient_authority());
    Box::new(
        <cap_rand::rngs::StdRng as cap_rand::SeedableRng>::from_seed(cap_rand::Rng::gen(&mut rng)),
    )
}
