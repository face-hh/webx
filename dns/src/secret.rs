use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn generate(size: usize) -> String {
    const ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet_len = ALPHABET.len();
    let mask = alphabet_len.next_power_of_two() - 1;
    let step = 8 * size / 5;

    let mut id = String::with_capacity(size);
    let mut rng = StdRng::from_entropy();

    while id.len() < size {
        let bytes: Vec<u8> = (0..step).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        id.extend(
            bytes
                .iter()
                .map(|&byte| (byte as usize) & mask)
                .filter_map(|index| ALPHABET.get(index).copied())
                .take(size - id.len())
                .map(char::from),
        );
    }

    id
}
