use std::num::ParseIntError;

pub fn str_to_id(s: &str) -> Result<usize, ParseIntError> {
    s.parse::<usize>()
}

pub fn str_to_u32(s: &str) -> Result<u32, ParseIntError> {
    s.parse::<u32>()
}
