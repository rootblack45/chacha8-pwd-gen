use rand::{prelude::Distribution, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy)]
struct Alphanumeric;

impl Distribution<u8> for Alphanumeric {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 26 + 10 + 8; // upper-case and lower-case letters + numbers + special characters
        const GEN_ASCII_STR_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789\
                !@#$%^&*";
        loop {
            let var = rng.next_u32() >> (32 - 7); // "Uniform" maybe better
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

fn main() {
    let mut rng = ChaCha8Rng::from_entropy(); // 8-round is enough?
    let chars: String = (0..12).map(|_| rng.sample(Alphanumeric) as char).collect();
    println!("{}", chars);
}
