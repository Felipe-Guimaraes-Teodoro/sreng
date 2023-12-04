use rand::prelude::*;

pub struct Math {}

impl Math {
    pub fn random
        <
            T: std::cmp::PartialOrd +
            rand::distributions::uniform::SampleUniform,
        >
        (
            n1: T, 
            n2: T
        ) -> T {

        let mut r = thread_rng();
        r.gen_range(n1..n2)
    }
}
