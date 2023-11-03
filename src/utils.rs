use crate::{ENVELOPE_AMOUNT, OSCILLATOR_AMOUNT};

pub mod fixed_map;

fn get_vector(size: usize) -> Vec<usize> {
    (0..size).collect::<Vec<usize>>()
}

pub fn get_oscillator_array() -> [usize; OSCILLATOR_AMOUNT] {
    get_vector(OSCILLATOR_AMOUNT).try_into().unwrap()
}

pub fn get_envelope_array() -> [usize; ENVELOPE_AMOUNT] {
    get_vector(ENVELOPE_AMOUNT).try_into().unwrap()
}