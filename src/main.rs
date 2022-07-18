use rand::{distributions::Uniform, prelude::Distribution, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy)]
pub struct Alphanumeric;

impl Distribution<u8> for Alphanumeric {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10 + 8; // upper-case and lower-case letters + numbers + special characters
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789\
                !@#$%^&*";

        let range = Uniform::new(0, RANGE);
        let n = range.sample(rng);
        GEN_ASCII_STR_CHARSET[n as usize]
    }
}

fn main() {
    let mut rng = ChaCha8Rng::from_entropy(); // 8-round is enough?A

    let mut incorrect = false;
    for _ in 0..100 {
        let c: char = rng.sample(Alphanumeric).into();
        incorrect |= !(('0'..='9').contains(&c)
            || ('A'..='Z').contains(&c)
            || ('a'..='z').contains(&c)
            || (['!', '@', '#', '$', '%', '^', '&', '*']).contains(&c));
    }
    assert!(!incorrect);
}
