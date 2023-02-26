use rand::{seq::SliceRandom, Rng};

mod adjectives;
mod nouns;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let adjective = adjectives::ADJECTIVES.choose(&mut rng).unwrap();
    let noun = nouns::NOUNS.choose(&mut rng).unwrap();
    let number: u32 = rng.gen_range(1..9999);
    format!("{}_{}_{}", adjective, noun, number)
}
