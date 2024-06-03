use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn generate(size: usize) -> String {
    pub const ALPHABET: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
        'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let mask = ALPHABET.len().next_power_of_two() - 1;
    let step: usize = 8 * size / 5;

    let mut id = String::with_capacity(size);
    let mut rng = StdRng::from_entropy();

    while id.len() < size {
        let bytes: Vec<u8> = (0..step).map(|_| rng.gen()).collect();

        id.extend(
            bytes
                .into_iter()
                .map(|byte| byte as usize & mask)
                .filter_map(|index| ALPHABET.get(index).copied())
                .take(size - id.len()),
        );
    }

    return id;
}
